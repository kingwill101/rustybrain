import 'package:flutter/material.dart';

import 'package:mobile/question/question_interface.dart'
    if (dart.library.io) 'package:mobile/question/ffi.dart'
    if (dart.library.js) 'package:mobile/question/web.dart' show QuestionObject;

import 'package:flutter_svg/flutter_svg.dart';
import 'package:mobile/utils.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // Try running your application with "flutter run". You'll see the
        // application has a blue toolbar. Then, without quitting the app, try
        // changing the primarySwatch below to Colors.green and then invoke
        // "hot reload" (press "r" in the console where you ran "flutter run",
        // or simply save your changes to "hot reload" in a Flutter IDE).
        // Notice that the counter didn't reset back to zero; the application
        // is not restarted.
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late QuestionObject question;

  void updateQuestion() {
    setState(() {
      question = QuestionObject();
    });
  }

  @override
  void initState() {
    super.initState();
    updateQuestion();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(question.name),
      ),
      body: _body(),
      floatingActionButton: FloatingActionButton(
        child: const Icon(Icons.query_stats_outlined),
        onPressed: updateQuestion,
      ),
    );
  }

  Widget _body() {
    return Stack(
      children: <Widget>[
        SvgPicture.asset(
          'assets/themes/notebook_background.svg',
          alignment: Alignment.center,
          width: MediaQuery.of(context).size.width,
          height: MediaQuery.of(context).size.height,
          fit: BoxFit.fill,
        ),
        Column(
          mainAxisAlignment: MainAxisAlignment.start,
          mainAxisSize: MainAxisSize.max,
          children: <Widget>[
            Container(
              margin: const EdgeInsets.all(30),
              padding: const EdgeInsets.all(8.0),
              child: Text(
                question.question,
                style: const TextStyle(fontSize: 30),
              ),
            ),
            Text(question.rationale),
            objectsToWidget(question),
            FutureBuilder(
                builder: (context, snapshot) => question.image.path!.isEmpty
                    ? const SizedBox()
                    : SvgPicture.asset(
                        "assets/game-graphics/${question.image.path}"))

            // ignore: prefer_const_literals_to_create_immutables
          ],
        ),
      ],
    );
  }
}
