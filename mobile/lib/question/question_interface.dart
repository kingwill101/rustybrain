import 'package:mobile/game_object.dart';

class Question {
  String _question = "";

  String _rationale = "";

  String _name = "";

  ImageObject? _image;

  List<GameObject> _drawables = [];

  String get question => _question;

  set question(q) {
    _question = q;
  }

  String get rationale => _rationale;

  set rationale(r) {
    _rationale = r;
  }

  String get name => _name;

  set name(r) {
    _name = r;
  }

  ImageObject get image => _image!;

  set image(i) {
    _image = i;
  }

  List<GameObject> get drawables => _drawables;

  set drawables(r) {
    _drawables = r;
  }

  String getPrefix(int index, String ans) {
    return ans;
  }
}

class QuestionObject extends Question {}
