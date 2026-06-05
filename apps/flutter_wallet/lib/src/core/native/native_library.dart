import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:isolate';

import 'package:ffi/ffi.dart';

import '../../wallet_api.dart';

typedef NativeWalletLibrary = ffi.DynamicLibrary;
typedef _WalletCommandNative = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);
typedef _WalletCommandDart = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);
typedef _WalletStringFreeNative = ffi.Void Function(ffi.Pointer<Utf8>);
typedef _WalletStringFreeDart = void Function(ffi.Pointer<Utf8>);
typedef NativeCommandRunner =
    Future<Map<String, dynamic>> Function(Map<String, Object?> command);

class LocalNativeCommandRunner {
  LocalNativeCommandRunner(NativeWalletLibrary library) {
    try {
      _command = library
          .lookupFunction<_WalletCommandNative, _WalletCommandDart>(
            'wallet_command_json_c',
          );
      _free = library
          .lookupFunction<_WalletStringFreeNative, _WalletStringFreeDart>(
            'wallet_string_free',
          );
    } catch (_) {
      throw const WalletApiException('Native wallet library unavailable');
    }
  }

  late final _WalletCommandDart _command;
  late final _WalletStringFreeDart _free;

  Future<Map<String, dynamic>> call(Map<String, Object?> command) async {
    return _invokeNativeCommand(command, _command, _free);
  }
}

Future<Map<String, dynamic>> runNativeCommandInBackground(
  Map<String, Object?> command,
) {
  return Isolate.run(() => _invokeNativeCommandWithDefaultLibrary(command));
}

Map<String, dynamic> _invokeNativeCommandWithDefaultLibrary(
  Map<String, Object?> command,
) {
  final library = _openDefaultLibrary();
  final commandFunction = library
      .lookupFunction<_WalletCommandNative, _WalletCommandDart>(
        'wallet_command_json_c',
      );
  final freeFunction = library
      .lookupFunction<_WalletStringFreeNative, _WalletStringFreeDart>(
        'wallet_string_free',
      );
  return _invokeNativeCommand(command, commandFunction, freeFunction);
}

Map<String, dynamic> _invokeNativeCommand(
  Map<String, Object?> command,
  _WalletCommandDart commandFunction,
  _WalletStringFreeDart freeFunction,
) {
  final commandPointer = jsonEncode(command).toNativeUtf8();
  ffi.Pointer<Utf8> responsePointer = ffi.nullptr;
  try {
    responsePointer = commandFunction(commandPointer);
    if (responsePointer == ffi.nullptr) {
      throw const WalletApiException('Native command failed');
    }
    final responseJson = responsePointer.toDartString();
    return jsonDecode(responseJson) as Map<String, dynamic>;
  } on WalletApiException {
    rethrow;
  } catch (_) {
    throw const WalletApiException('Native command failed');
  } finally {
    calloc.free(commandPointer);
    if (responsePointer != ffi.nullptr) {
      freeFunction(responsePointer);
    }
  }
}

ffi.DynamicLibrary _openDefaultLibrary() {
  try {
    if (Platform.isIOS) {
      return ffi.DynamicLibrary.process();
    }
    if (Platform.isMacOS) {
      return ffi.DynamicLibrary.open(_macosPackagedLibraryPath());
    }
    if (Platform.isAndroid) {
      return ffi.DynamicLibrary.open('libwallet_ffi.so');
    }
  } catch (_) {
    throw const WalletApiException('Native wallet library unavailable');
  }
  throw const WalletApiException('Unsupported wallet platform');
}

String _macosPackagedLibraryPath() {
  final executable = File(Platform.resolvedExecutable);
  final macosDirectory = executable.parent;
  final contentsDirectory = macosDirectory.parent;
  return '${contentsDirectory.path}/Frameworks/libwallet_ffi.dylib';
}
