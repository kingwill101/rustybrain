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

  @override
  String get question {
    if (question_.isNotEmpty) {
      return question_;
    }

    var rustString = library.engine_context_get_question(context);

    question_ = rustString.cast<Utf8>().toDartString();
    library.engine_free_string(rustString);

    return question_;
  }

  @override
  String get rationale {
    if (rationale_.isNotEmpty) {
      return rationale_;
    }

    var rustString = library.engine_context_get_rationale(context);

    rationale_ = rustString.cast<Utf8>().toDartString();
    library.engine_free_string(rustString);

    return rationale_;
  }

  @override
  String get name {
    if (name_.isEmpty) {
      var rustString = library.engine_context_get_name(context);

      name_ = rustString.cast<Utf8>().toDartString();
      library.engine_free_string(rustString);
    }

    return name_;
  }

  @override
  List<GameObject> get drawables {
    var rustString = library.engine_context_get_drawables(context);

    var drawableString = rustString.cast<Utf8>().toDartString();

    drawables_ = (jsonDecode(drawableString) as List)
        .map((e) => GameObject.fromJson(e))
        .toList();

    library.engine_free_string(rustString);

    return drawables_;
  }

  @override
  String getPrefix(int index, String ans) {
    var dartNativeString = ans.toNativeUtf8().cast<Int8>();

    var str = library.engine_context_get_option_prefix(
        context, index, dartNativeString);

    var value = str.cast<Utf8>().toDartString();
    library.engine_free_string(str);
    return value;
  }

  @override
  String interop(String s) {
    var dartNativeString = s.toNativeUtf8().cast<Int8>();

    var str = library.engine_context_string_interop(context, dartNativeString);

    var value = str.cast<Utf8>().toDartString();
    library.engine_free_string(str);
    return value;
  }
}
