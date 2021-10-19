import 'dart:convert';
import 'dart:ffi';
import 'package:more/more.dart';
import 'package:ffi/ffi.dart';
import 'package:flutter/widgets.dart';
import 'package:mobile/game_object.dart';
import 'package:mobile/rustybrain_object.dart';

class QuestionObject extends RustyBrainObject {
  QuestionObject({context, library})
      : super(context: context, librustyBrain: library);
  String? _question;
  String? _rationale;
  String? _name;
  ImageObject? _image;
  List<GameObject>? _drawables;

  String _getQuestion() {
    var rustString = lib.engine_context_get_question(context);

    _question = rustString.cast<Utf8>().toDartString();
    lib.engine_free_string(rustString);

    return _question!;
  }

  String _getRationale() {
    var rustString = lib.engine_context_get_rationale(context);

    _rationale = rustString.cast<Utf8>().toDartString();
    lib.engine_free_string(rustString);

    return _rationale!;
  }

  String _getName() {
    var rustString = lib.engine_context_get_name(context);

    _name = rustString.cast<Utf8>().toDartString();
    lib.engine_free_string(rustString);

    return _name!;
  }

  ImageObject _getImage() {
    var rustString = lib.engine_context_get_image(context);

    var _imagejson = rustString.cast<Utf8>().toDartString();
    lib.engine_free_string(rustString);
    _image =
        (ImageObject.fromJson(jsonDecode(_imagejson) as Map<String, dynamic>));

    return _image!;
  }

  List<GameObject> _getDrawables() {
    var rustString = lib.engine_context_get_drawables(context);

    var drawableString = rustString.cast<Utf8>().toDartString();

    _drawables = (jsonDecode(drawableString) as List)
        .map((e) => GameObject.fromJson(e))
        .toList();

    lib.engine_free_string(rustString);
    return _drawables!;
  }

  String get question => _question != null ? _question! : _getQuestion();

  String get rationale => _rationale != null ? _rationale! : _getRationale();

  String get name => _name != null ? _name! : _getName();

  ImageObject get image => _image != null ? _image! : _getImage();

  List<GameObject> get drawables =>
      _drawables != null ? _drawables! : _getDrawables();

  String get_prefix(int index, String ans) {
    var dartNativeString = ans.toNativeUtf8().cast<Int8>();

    var str =
        lib.engine_context_get_option_prefix(context, index, dartNativeString);

    var value = str.cast<Utf8>().toDartString();
    lib.engine_free_string(str);
    return value;
  }
}

Widget objectsToWidget(QuestionObject q) {
  // var parent = Container();
  List<Widget> items = [];

  print(q.image);
  q.drawables
      .where((element) => element.isOption == false)
      .toList()
      .indexed()
      .forEach((element) {
    items.add(Text(
        q.get_prefix(
            element.index,
            element.value.text!.text!.plural!.isEmpty
                ? element.value.text!.text!.singular!
                : element.value.text!.text!.plural!),
        style: const TextStyle(fontSize: 22)));
  });

  q.drawables
      .where((element) => element.isOption == true)
      .toList()
      .indexed()
      .forEach((element) {
    items.add(Text(
      q.get_prefix(
          element.index,
          element.value.text!.text!.plural!.isEmpty
              ? element.value.text!.text!.singular!
              : element.value.text!.text!.plural!),
      style: const TextStyle(fontSize: 25, fontWeight: FontWeight.bold),
    ));
  });

  return Column(
    children: items,
  );
}
