import 'package:dart_json_mapper/dart_json_mapper.dart';

@jsonSerializable
class InputProperties {
  InputProperties(this._properties);
  Map<String, dynamic> _properties = {};

  @jsonProperty
  void unmappedSet(String name, dynamic value) {
    _properties[name] = value;
  }

  @jsonProperty
  Map<String, dynamic> unmappedGet() {
    return _properties;
  }
}
