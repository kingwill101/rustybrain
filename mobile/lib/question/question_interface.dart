import 'package:mobile/game_object.dart';

abstract class Question {
  String question_ = "";

  String rationale_ = "";

  String name_ = "";

  List<GameObject> drawables_ = [];

  String get question => question_;

  set question(q) {
    question_ = q;
  }

  String get rationale => rationale_;

  set rationale(r) {
    rationale_ = r;
  }

  String get name => name_;

  set name(r) {
    name_ = r;
  }

  List<GameObject> get drawables => drawables_;

  set drawables(r) {
    drawables_ = r;
  }

  String getPrefix(int index, String ans) {
    return ans;
  }
}

class QuestionObject extends Question {}
