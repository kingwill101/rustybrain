@JS()
library wasm.pkg.librustybrain_wasm;

import "package:js/js.dart";
import "package:js/js_util.dart" show promiseToFuture;

/// tslint:disable
/// eslint-disable
@JS()
external bool init_manager();
@JS()
external WrappedContext new_context();
@JS()
external WrappedContext new_by_category(String cat);
@JS()
external WrappedContext new_by_category_name(String cat, String name);
@JS()
external WrappedContext new_context_by_category(String cat);
@JS()
external void main();
@JS()
external num add(num a, num b);

@JS()
class MANAGER {
  // @Ignore
  external MANAGER.fakeConstructor$();
  external void free();
}

@JS()
class WrappedContext {
  // @Ignore
  external WrappedContext.fakeConstructor$();
  external void free();
  external factory WrappedContext();
  external String string_interop(String s);
  external String get_question();
  external String get_rationale();
  external String get_name();
  external String get_drawables();
  external String get_option_prefix(
      dynamic /*num|dynamic*/ index, String content);
}

// /*export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;*/
// @anonymous
// @JS()
// abstract class InitOutput {
//   external Memory get memory;
//   external num Function() get init_manager;
//   external void Function(num) get JS$__wbg_wrappedcontext_free;
//   external num Function() get wrappedcontext_new;
//   external void Function(num, num) get wrappedcontext_get_question;
//   external void Function(num, num) get wrappedcontext_get_rationale;
//   external void Function(num, num) get wrappedcontext_get_name;
//   external void Function(num, num) get wrappedcontext_get_drawables;
//   external void Function(num, num, num, num, num)
//       get wrappedcontext_get_option_prefix;
//   external num Function() get new_context;
//   external num Function(num, num) get new_context_by_category;
//   external void Function() get main;
//   external void Function(num) get JS$__wbg_manager_free;
//   external num Function(num, num) get add;
//   external num Function(num) get JS$__wbindgen_add_to_stack_pointer;
//   external void Function(num, num) get JS$__wbindgen_free;
//   external num Function(num) get JS$__wbindgen_malloc;
//   external num Function(num, num, num) get JS$__wbindgen_realloc;
//   external void Function(num) get JS$__wbindgen_exn_store;
//   external void Function() get JS$__wbindgen_start;
//   external factory InitOutput(
//       {Memory memory,
//       num Function() init_manager,
//       num Function() wrappedcontext_new,
//       void Function(num, num) wrappedcontext_get_question,
//       void Function(num, num) wrappedcontext_get_rationale,
//       void Function(num, num) wrappedcontext_get_name,
//       void Function(num, num) wrappedcontext_get_drawables,
//       void Function(num, num, num, num, num) wrappedcontext_get_option_prefix,
//       num Function() new_context,
//       num Function(num, num) new_context_by_category,
//       void Function() main,
//       num Function(num, num) add});
// }

@JS()
abstract class Promise<T> {
  external factory Promise(
      void executor(void resolve(T result), Function reject));
  external Promise then(void onFulfilled(T result), [Function onRejected]);
}

