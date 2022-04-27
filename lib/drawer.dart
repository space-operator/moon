import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:path_provider/path_provider.dart';

import 'package:moon/commands/const_subblocks/file_picker.dart';

import 'package:moon/utils/clipboard.dart';
import 'package:moon/utils/launch_url_in_browser.dart';
import 'package:moon/utils/logger.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'package:intl/intl.dart';
import 'package:url_launcher/url_launcher.dart';

import 'package:file_picker/file_picker.dart';

Widget insertTextButton(String text) {
  return SizedBox(
    width: 300,
    height: 50,
    child: Padding(
      padding: const EdgeInsets.fromLTRB(5, 15, 0, 0),
      child: Text(text),
    ),
  );
}

class DrawerWidget extends HookConsumerWidget {
  const DrawerWidget({
    required Key? key,
    required this.menuScroll,
    required this.store,
    required this.debugScrollController,
    required this.debug,
    required this.renameTextEditingController,
    required this.graph_entry,
    required this.graphList,
  }) : super(key: key);

  final ScrollController menuScroll;
  final rid.Store store;
  final ScrollController debugScrollController;
  final ValueNotifier<String> debug;
  final TextEditingController renameTextEditingController;
  final rid.GraphEntry graph_entry;
  final List<rid.GraphEntry> graphList;

  @override
  Widget build(BuildContext buildContext, WidgetRef ref) {
    return Drawer(
      backgroundColor: Color.fromARGB(255, 23, 30, 34),
      child: SingleChildScrollView(
        controller: menuScroll,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisAlignment: MainAxisAlignment.start,
          children: [
            Image.asset("assets/logo-full-small.png"),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.mouse),
              label: insertTextButton("Set Input For Mouse"),
              onPressed: () {
                ref
                    .read(storeRepoProvider)
                    .store
                    .msgSetMappingKind("mouse", timeout: Duration(minutes: 1))
                    .then((value) => Navigator.of(buildContext).pop());
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.laptop),
              label: insertTextButton("Set Input For Mac Trackpad"),
              onPressed: () {
                ref
                    .read(storeRepoProvider)
                    .store
                    .msgSetMappingKind("touch", timeout: Duration(minutes: 1))
                    .then((value) => Navigator.of(buildContext).pop());
                ;
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.save),
              label: insertTextButton("Export"),
              onPressed: () async {
                DateTime now = DateTime.now();
                String formattedDate =
                    DateFormat("yyyy-MM-dd--hhmmaa").format(now).toLowerCase();

                final filename = ref.read(storeRepoProvider).graph_entry.name +
                    " - " +
                    formattedDate;
                log.v(filename);
                String? path = await FilePicker.platform.saveFile(
                    fileName: filename,
                    type: FileType.custom,
                    allowedExtensions: ["json"]);

                if (path != null) {
                  log.v(path);
                  store
                      .msgExport(path, filename, timeout: Duration(minutes: 10))
                      .then((value) => Navigator.of(buildContext).pop());
                }
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.folder_open_rounded),
              label: insertTextButton("Import"),
              onPressed: () {
                filePicker(FileType.custom, jsonOnlyExtension,
                    PickerFollowAction.Import, ref);
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.bug_report_outlined),
              label: insertTextButton("Debug"),
              onPressed: () {
                Future<void> _showMyDialog() async {
                  return showDialog<void>(
                    context: buildContext,
                    barrierDismissible: true, // user must tap button!
                    builder: (BuildContext context) {
                      return AlertDialog(
                        title: const Text('Debug'),
                        content: SingleChildScrollView(
                          controller: debugScrollController,
                          child: SelectableText(
                            debug.value,
                            // maxLines: 8,
                          ),
                        ),
                        actions: <Widget>[
                          TextButton(
                            style: TextButton.styleFrom(
                                // primary: Colors.blueGrey[300],
                                backgroundColor: Colors.green,
                                shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(10))),
                            child: const Text('Copy',
                                style: const TextStyle(
                                  color: Colors.white,
                                )),
                            onPressed: () {
                              copyToClipboard(debug.value, buildContext);
                              Navigator.of(context).pop();
                            },
                          ),
                          const VerticalDivider(
                            width: 60,
                          ),
                          TextButton(
                            child: const Text('Close',
                                style: const TextStyle(color: Colors.blueGrey)),
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                          ),
                        ],
                      );
                    },
                  );
                }

                store
                    .msgDebug("debug", timeout: Duration(minutes: 1))
                    .then((value) {
                  debug.value = value.data!;
                }).then((value) => _showMyDialog());
              },
            ),
            const Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.open_in_new),
              label: insertTextButton("Open Run Logs Folder"),
              onPressed: () async {
                final persistentDir = await getApplicationSupportDirectory();
                final String uriPath =
                    persistentDir.absolute.path + "/run_logs";

                final Uri uri = Uri.file(uriPath);
                if (!await launch(uri.toString()))
                  throw 'Could not launch $uri';

                Navigator.of(buildContext).pop();
              },
            ),
            const Padding(
              padding: const EdgeInsets.all(8.0),
              child: const Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.drive_file_rename_outline),
              label: insertTextButton("Rename Graph"),
              onPressed: () {
                Future<void> _showMyDialog() async {
                  return showDialog<void>(
                    context: buildContext,
                    barrierDismissible: true, // user must tap button!
                    builder: (BuildContext context) {
                      return AlertDialog(
                        title: const Text('Rename Graph'),
                        content: SingleChildScrollView(
                          child: ListBody(
                            children: <Widget>[
                              TextField(
                                controller: renameTextEditingController,
                              )
                            ],
                          ),
                        ),
                        actions: <Widget>[
                          TextButton(
                            style: TextButton.styleFrom(
                                // primary: Colors.blueGrey[300],
                                backgroundColor: Colors.green,
                                shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(10))),
                            child: const Text('Rename',
                                style: const TextStyle(
                                  color: Colors.white,
                                )),
                            onPressed: () {
                              store.msgRenameGraph(graph_entry.id,
                                  renameTextEditingController.text,
                                  timeout: Duration(minutes: 1));

                              Navigator.of(context).pop();
                            },
                          ),
                          VerticalDivider(
                            width: 60,
                          ),
                          TextButton(
                            child: const Text('Cancel',
                                style: TextStyle(color: Colors.blueGrey)),
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                          ),
                        ],
                      );
                    },
                  );
                }

                _showMyDialog();
              },
            ),
            const Padding(
              padding: const EdgeInsets.all(8.0),
              child: const Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.delete_outline),
              label: insertTextButton(
                "Delete Graph",
              ),
              onPressed: () {
                Future<void> _showMyDialog() async {
                  return showDialog<void>(
                    context: buildContext,
                    barrierDismissible: true, // user must tap button!
                    builder: (BuildContext context) {
                      return AlertDialog(
                        title: const Text('Delete Graph'),
                        content: SingleChildScrollView(
                          child: ListBody(
                            children: <Widget>[
                              Text(
                                  "You are about to delete: ${graph_entry.name}"),
                            ],
                          ),
                        ),
                        actions: <Widget>[
                          TextButton(
                            style: TextButton.styleFrom(
                                // primary: Colors.blueGrey[300],
                                backgroundColor: Colors.red,
                                shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(10))),
                            child: const Text('DELETE',
                                style: const TextStyle(
                                  color: Colors.white,
                                )),
                            onPressed: () {
                              // print(graphList.length);
                              if (graphList.length == 0) {
                                store.msgDeleteGraph(graph_entry.id,
                                    timeout: Duration(minutes: 1));

                                store
                                    .msgLoadGraph("new",
                                        timeout: Duration(minutes: 1))
                                    .then(
                                        (value) => Navigator.of(context).pop())
                                    .then((value) => store.msgResetZoom(""));
                              }
                              if (graphList.length > 0) {
                                store.msgDeleteGraph(graph_entry.id,
                                    timeout: Duration(minutes: 1));
                                final loadGraph = graphList.firstWhere(
                                  (element) => element.id != graph_entry.id,
                                );
                                store
                                    .msgLoadGraph(loadGraph.id,
                                        timeout: Duration(minutes: 1))
                                    .then(
                                        (value) => Navigator.of(context).pop())
                                    .then((value) =>
                                        store.msgFitNodesToScreen(""));
                              }
                              // Navigator.of(context).pop();
                            },
                          ),
                          const VerticalDivider(
                            width: 60,
                          ),
                          TextButton(
                            child: const Text('Cancel',
                                style: const TextStyle(color: Colors.blueGrey)),
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                          ),
                        ],
                      );
                    },
                  );
                }

                _showMyDialog();
              },
            ),
            Container(
              height: 60,
            ),
            const Center(
              child: const Text(
                "Space Operator, Alpha Version",
                style: const TextStyle(color: Colors.white),
              ),
            ),
            Center(
              child: TextButton(
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(0, 15, 0, 0),
                  child: const Text(
                    "@space_operator_",
                    style: const TextStyle(color: Colors.white),
                  ),
                ),
                onPressed: () =>
                    launchInBrowser("https://twitter.com/space_operator_"),
              ),
            )
          ],
        ),
      ),
    );
  }
}
