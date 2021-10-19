import 'dart:ffi';

import 'package:mobile/generated_bindings.dart';

class RustyBrainObject {
  late Librustybrain lib;
  late Pointer<GameContext> context;

  RustyBrainObject(
      {Pointer<GameContext>? context, Librustybrain? librustyBrain}) {
    lib = librustyBrain ??
        Librustybrain(DynamicLibrary.open(
            "/home/kingwill101/code/rustybrain/target/debug/libffi.so"));

    if (!lib.engine_init_game_manager()) {
      throw Exception("Unable to initiate game manager");
    }
    this.context = context ?? lib.engine_context_new();
  }
}
