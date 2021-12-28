import 'dart:convert';

import 'package:mobile/game_object.dart';
import 'package:mobile/question/question_interface.dart';
import 'package:rustybrain/rustybrain.dart' as rustybrain;

class QuestionObject extends Question {
  late rustybrain.WrappedContext contextHolder;

  QuestionObject() {
    assert(rustybrain.init_manager() == true);

    contextHolder = rustybrain.new_context();
  }

  @override
  get question {
    if (question_.isNotEmpty) {
      return question_;
    }

    question_ = contextHolder.get_question();

    return question_;
  }

  @override
  String get rationale {
    if (rationale_.isNotEmpty) {
      return rationale_;
    }

    rationale_ = contextHolder.get_rationale();

    return rationale_;
  }

  @override
  String get name {
    if (name_.isNotEmpty) {
      return name_;
    }

    name_ = contextHolder.get_name();

    return name_;
  }

  @override
  List<GameObject> get drawables {
    var drawableString = contextHolder.get_drawables();

    drawables_ = (jsonDecode(drawableString) as List)
        .map((e) => GameObject.fromJson(e))
        .toList();

    return drawables_;
  }

  @override
  String getPrefix(int index, String ans) {
    return contextHolder.get_option_prefix(index, ans);
  }

  @override
  String interop(String s) {
    return contextHolder.string_interop(s);
  }
}
