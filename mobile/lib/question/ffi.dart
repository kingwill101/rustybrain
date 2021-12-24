import 'dart:convert';
import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:mobile/game_object.dart';
import 'package:mobile/generated_bindings.dart';
import 'package:mobile/question/question_interface.dart';

class QuestionObject extends Question {
  late Pointer<GameContext> context;
  late Librustybrain library;

  QuestionObject() {
    library = Librustybrain(DynamicLibrary.open(
        "/home/kingwill101/code/rustybrain/target/debug/libffi.so"));

    if (!library.engine_init_game_manager()) {
      throw Exception("Unable to initiate game manager");
    }
    context = library.engine_context_new();
  }

  String _getQuestion() {
    var rustString = library.engine_context_get_question(context);

    question = rustString.cast<Utf8>().toDartString();
    library.engine_free_string(rustString);

    return question;
  }

  String _getRationale() {
    var rustString = library.engine_context_get_rationale(context);

    rationale = rustString.cast<Utf8>().toDartString();
    library.engine_free_string(rustString);

    return rationale;
  }

  String _getName() {
    var rustString = library.engine_context_get_name(context);

    name = rustString.cast<Utf8>().toDartString();
    library.engine_free_string(rustString);

    return name;
  }

  ImageObject _getImage() {
    var rustString = library.engine_context_get_image(context);

    var _imagejson = rustString.cast<Utf8>().toDartString();
    library.engine_free_string(rustString);
    image =
        (ImageObject.fromJson(jsonDecode(_imagejson) as Map<String, dynamic>));

    return image;
  }

  List<GameObject> _getDrawables() {
    var rustString = library.engine_context_get_drawables(context);

    var drawableString = rustString.cast<Utf8>().toDartString();

    drawables = (jsonDecode(drawableString) as List)
        .map((e) => GameObject.fromJson(e))
        .toList();

    library.engine_free_string(rustString);

    return drawables;
  }

  @override
  String get question => _getQuestion();

  @override
  String get rationale => _getRationale();

  @override
  String get name => _getName();

  @override
  ImageObject get image => _getImage();

  @override
  List<GameObject> get drawables => _getDrawables();

  @override
  String getPrefix(int index, String ans) {
    var dartNativeString = ans.toNativeUtf8().cast<Int8>();

    var str = library.engine_context_get_option_prefix(
        context, index, dartNativeString);

    var value = str.cast<Utf8>().toDartString();
    library.engine_free_string(str);
    return value;
  }
}
