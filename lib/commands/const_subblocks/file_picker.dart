import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/commands/const.dart';
import 'package:path/path.dart' as p;

import 'package:moon/providers/store_provider.dart';
// import 'package:flutter_json_view/flutter_json_view.dart';

@immutable
class FileState {
  const FileState({
    this.file,
    this.loadedFromDb = "",
    this.extension = "",
    this.path = "",
    this.size = 0,
  });

  final File? file;
  final String loadedFromDb;
  final String extension;
  final String path;
  final int size;
}

class ReadFile {
  ReadFile({this.treeNodeText = "", this.loadedFrom = ""}) {
    print("read file ${this.treeNodeText}");
    if (this.treeNodeText != "") {
      this.path = jsonDecode(treeNodeText!)["Const"]["String"];
      this.loadedFrom = "db";
      this.file = File(this.path!);
      // get the extension and remove the dot
      this.extension = p.extension(this.path!).replaceRange(0, 1, "");
      this.size = file!.statSync().size;
    }
  }

  ReadFile.picker(
      {this.path,
      this.extension,
      this.file,
      this.size,
      this.loadedFrom = "picker"});

  String? extension;
  String? loadedFrom;
  String? treeNodeText;
  String? path;
  File? file;
  int? size;
}

class FilePickerField extends HookConsumerWidget {
  FilePickerField({Key? key, required this.treeNode, required this.focusNode})
      : super(key: key);

  final TreeNode treeNode;
  final FocusNode focusNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final store = ref.read(storeRepoProvider).store;

    ValueNotifier<bool> decodeSucceeded = useState(false);

    FileState _reducer(FileState state, ReadFile action) {
      if (action.loadedFrom == "db") {
        return FileState(
          file: action.file,
          extension: action.extension!,
          path: action.path!,
          size: action.size!,
        );
      }
      if (action.loadedFrom == "picker") {
        return FileState(
          file: action.file,
          extension: action.extension!,
          path: action.path!,
          size: action.size!,
        );
      }
      return state;
    }

    final _store = useReducer<FileState, ReadFile>(
      _reducer,
      initialState: const FileState(),
      initialAction: ReadFile(treeNodeText: treeNode.node.value.text),
    );

    saveToDb() {
      if (decodeSucceeded.value == true) {
        final text = _store.state.file!.path;
        final inputEvent = createJson(
          text,
          treeNode.node.key,
          "String",
        );
        store.msgSendJson(inputEvent, timeout: Duration(minutes: 1));
      }
    }

    return Expanded(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          if (_store.state.file == null)
            Expanded(
              child: ElevatedButton(
                style: TextButton.styleFrom(
                    primary: Colors.blueGrey[300],
                    backgroundColor: Colors.grey[300]),
                onPressed: () async {
                  decodeSucceeded.value = false;

                  final pickedFile = await filePicker(
                      FileType.any, null, PickerFollowAction.Save, ref);
                  // print(file);
                  if (pickedFile != null) {
                    File file = File(pickedFile.path!);

                    _store.dispatch(ReadFile.picker(
                      file: file,
                      extension: pickedFile.extension!,
                      path: pickedFile.path!,
                      size: pickedFile.size,
                    ));
                    decodeSucceeded.value = true;
                    saveToDb();
                  }
                },
                child: Center(child: const Icon(Icons.folder_open_outlined)),
              ),
            ),
          if (_store.state.file != null) ...[
            Row(
              children: [
                Expanded(
                  child: SingleChildScrollView(
                    child: Container(
                      decoration: BoxDecoration(
                        color: Color(0xFFEEEEEE),
                        // border: Border.all(
                        //   color: Color(0xFF258ED5),
                        // ),
                      ),
                      height: 40,
                      child: Text(
                        _store.state.path,
                      ),
                    ),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.folder_open_rounded),
                  onPressed: () async {
                    decodeSucceeded.value = false;

                    final pickedFile = await filePicker(
                        FileType.any, null, PickerFollowAction.Save, ref);
                    // print(file);
                    if (pickedFile != null) {
                      File file = File(pickedFile.path!);

                      _store.dispatch(ReadFile.picker(
                        file: file,
                        extension: pickedFile.extension!,
                        path: pickedFile.path!,
                        size: pickedFile.size,
                      ));

                      decodeSucceeded.value = true;
                      saveToDb();
                    }
                  },
                ),
              ],
            ),
            Text(formatBytes(_store.state.size)),
            if (_store.state.file != null)
              imageExtensions.contains(_store.state.extension.toLowerCase())
                  ? Expanded(
                      child: Image.file(
                        _store.state.file!,
                        fit: BoxFit.contain,
                      ),
                    )
                  : Expanded(
                      child: Center(child: Text("preview not yet supported"))),
          ],
        ],
      ),
    );
  }
}

String formatBytes(int bytes, {int decimals = 2}) {
  if (bytes <= 0) return "0 b";
  const suffixes = ["b", "kb", "Mb", "Gb", "tb", "pb", "eb", "zb", "yb"];
  var i = (log(bytes) / log(1000)).floor();
  return ((bytes / pow(1000, i)).toStringAsFixed(decimals)) + ' ' + suffixes[i];
}

enum PickerFollowAction {
  Path,
  Import,
  Save,
}

const jsonOnlyExtension = ["json"];
const imageExtensions = ["jpeg", "jpg", "png", "gif", "webp", "bmp", "wbmp"];

Future<PlatformFile?> filePicker(FileType fileType, List<String>? extensions,
    PickerFollowAction action, WidgetRef ref) async {
  // PlatformFile? file;

  final FutureOr<PlatformFile?> platformFile = await FilePicker.platform
      .pickFiles(type: fileType, allowedExtensions: extensions, withData: true)
      .then(
    (result) {
      if (result != null) {
        final file = PlatformFile(
          path: result.files.single.path,
          name: result.files.single.name,
          size: result.files.single.size,
          bytes: result.files.single.bytes,
        );

        switch (action) {
          case PickerFollowAction.Import:
            {
              final store = ref.read(storeRepoProvider).store;
              store
                  .msgImport(file.path!, timeout: Duration(hours: 8))
                  .then((value) => store.msgFitNodesToScreen(""));
              return Future.value(file);
            }
          case PickerFollowAction.Save:
            {
              // save to db
              return Future.value(file);
            }
          default:
            return Future.value(file);
        }
      } else {
        // User canceled the picker
        return Future.value(null);
      }
    },
  );
  return platformFile;
}
