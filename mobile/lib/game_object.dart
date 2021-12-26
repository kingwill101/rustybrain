class GameObject {
  GameObject({
    required this.position,
    required this.dimensions,
    required this.order,
    required this.textObject,
    required this.isOption,
    required this.isCorrect,
    required this.isImage,
    required this.path,
  });

  late final Position position;
  late final Dimensions dimensions;
  late final String order;
  late final TextObject textObject;
  late final bool isOption;
  late final bool isCorrect;
  late final bool isImage;
  late final String path;

  GameObject.fromJson(Map<String, dynamic> json) {
    position = Position.fromJson(json['position']);
    dimensions = Dimensions.fromJson(json['dimensions']);
    order = json['order'];
    textObject = TextObject.fromJson(json['text_object']);
    isOption = json['is_option'];
    isCorrect = json['is_correct'];
    isImage = json['is_image'];
    path = json['path'];
  }

  Map<String, dynamic> toJson() {
    final _data = <String, dynamic>{};
    _data['position'] = position.toJson();
    _data['dimensions'] = dimensions.toJson();
    _data['order'] = order;
    _data['text_object'] = textObject.toJson();
    _data['is_option'] = isOption;
    _data['is_correct'] = isCorrect;
    _data['is_image'] = isImage;
    _data['path'] = path;
    return _data;
  }
}

class Position {
  Position({
    required this.x,
    required this.y,
  });

  late final double x;
  late final double y;

  Position.fromJson(Map<String, dynamic> json) {
    x = json['x'];
    y = json['y'];
  }

  Map<String, dynamic> toJson() {
    final _data = <String, dynamic>{};
    _data['x'] = x;
    _data['y'] = y;
    return _data;
  }
}

class Dimensions {
  Dimensions({
    required this.width,
    required this.height,
  });

  late final double width;
  late final double height;

  Dimensions.fromJson(Map<String, dynamic> json) {
    width = json['width'];
    height = json['height'];
  }

  Map<String, dynamic> toJson() {
    final _data = <String, dynamic>{};
    _data['width'] = width;
    _data['height'] = height;
    return _data;
  }
}

class TextObject {
  TextObject({
    required this.position,
    required this.text,
    required this.centered,
    required this.size,
  });

  late final Position position;
  late final Text text;
  late final bool centered;
  late final String size;

  TextObject.fromJson(Map<String, dynamic> json) {
    position = Position.fromJson(json['position']);
    text = Text.fromJson(json['text']);
    centered = json['centered'];
    size = json['size'];
  }

  Map<String, dynamic> toJson() {
    final _data = <String, dynamic>{};
    _data['position'] = position.toJson();
    _data['text'] = text.toJson();
    _data['centered'] = centered;
    _data['size'] = size;
    return _data;
  }
}

class Text {
  Text({
    required this.singular,
    required this.plural,
  });

  late final String singular;
  late final String plural;

  Text.fromJson(Map<String, dynamic> json) {
    singular = json['singular'];
    plural = json['plural'];
  }

  Map<String, dynamic> toJson() {
    final _data = <String, dynamic>{};
    _data['singular'] = singular;
    _data['plural'] = plural;
    return _data;
  }
}
