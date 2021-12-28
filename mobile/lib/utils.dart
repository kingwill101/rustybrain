import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';
import 'package:mobile/game_object.dart' as models;
import 'package:mobile/question/question_interface.dart';
import 'package:more/more.dart';

Widget objectsToStack(Question q, double width, double height) {
  List<Widget> items = [];

  var factor = 39;
  for (var element in q.drawables.indexed()) {
    if (element.value.isImage) {
      Widget image;
      var path = "assets/game-graphics/${element.value.path}";
      if (kDebugMode) {
        print("Image path: $path");
      }

      if (element.value.path.endsWith(".svg")) {
        image = SvgPicture.asset(
          path,
          // alignment: Alignment.center,
          fit: BoxFit.fitWidth,
          // width: element.value.dimensions.width * (width / 2),
          // height: element.value.dimensions.height * (height / 2),
        );
      } else {
        image = Image.asset(
          path,
        );
      }

      items.add(
        Positioned(
          top: element.value.position.y * (height / 2),
          left: element.value.position.x * (width / 2),
          child: SizedBox(
            width: element.value.dimensions.width * (width / 2),
            height: element.value.dimensions.height * (height / .99),
            child: image,
          ),
        ),
      );
    } else if (element.value.isOption) {
      items.add(
        Positioned(
          top: element.value.textObject.position.y * (height * .8) + factor,
          left: element.value.textObject.position.x * (width * .45),
          child: Text(
            q.getPrefix(element.index - 1, pluralize(element.value.textObject)),
            style: const TextStyle(fontSize: 22),
          ),
        ),
      );
    } else {
      items.add(
        Positioned(
          top: element.value.textObject.position.y * (height * .8) + factor,
          left: element.value.textObject.position.x * (width * .45),
          child: Text(
              q.interop(
                pluralize(element.value.textObject),
              ),
              style:
                  const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
        ),
      );
    }
  }

  return Stack(
      clipBehavior: Clip.none,
      alignment: Alignment.center,
      textDirection: TextDirection.ltr,
      children: [...items]);
}

String pluralize(models.TextObject textObject) {
  return textObject.text.plural.isEmpty
      ? textObject.text.singular
      : textObject.text.plural;
}
