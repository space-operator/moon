import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:moon/providers/help.dart';

import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/templates.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'dart:async';
import 'dart:io';
import 'dart:typed_data';
import 'package:path_provider/path_provider.dart';

class TitleArea extends HookConsumerWidget {
  const TitleArea({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final store = ref.read(storeRepoProvider).store;
    final templatesProvider = ref.read(templates);
    final helpProvider = ref.read(help);
    final graphProvider = ref.watch(graphController);
    final graph_entry = ref.read(storeRepoProvider).graph_entry;

    final showButtonText =
        MediaQuery.of(context).size.width > 1600 ? true : false;
    // create dropdown options
    final List<rid.GraphEntry> graphList =
        ref.read(storeRepoProvider).graph_list;

    // remove current graph to re-add it at the top
    graphList.removeWhere((entry) => entry.id == graph_entry.id);
    List<DropdownMenuItem<String>> dropDownList = graphList.map(
      (e) {
        return DropdownMenuItem(child: Text(e.name), value: e.id);
      },
    ).toList();

    dropDownList.insert(
        0, DropdownMenuItem(child: Text("+ New Flow"), value: "new"));
    dropDownList.insert(1,
        DropdownMenuItem(child: Text(graph_entry.name), value: graph_entry.id));

    return Container(
      child: Row(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(0, 0, 10, 0),
            child: showButtonText
                ? Text("Flow Name:",
                    style: TextStyle(color: Colors.blueGrey.shade400))
                : const Text(""),
          ),
          DropdownButton(
              style: TextStyle(
                color: Colors.amber,
              ),
              dropdownColor: Colors.blueGrey.shade900,
              alignment: AlignmentDirectional.bottomCenter,
              items: dropDownList,
              value: graph_entry.id,
              onChanged: (value) {
                store
                    .msgLoadGraph(value.toString(),
                        timeout: Duration(minutes: 1))
                    .then((val) {
                  if (value.toString() == "new") {
                    store.msgResetZoom("");
                  } else {
                    store.msgFitNodesToScreen("");
                  }
                });
              }),
          const Padding(
            padding: const EdgeInsets.all(8.0),
            child: const VerticalDivider(color: Colors.white),
          ),
          TextButton.icon(
            style: TextButton.styleFrom(
              primary: Colors.blueGrey.shade400,
            ),
            icon: const Icon(Icons.import_contacts_outlined),
            label: showButtonText ? const Text("load template") : Text(""),
            onPressed: () {
              Future<void> _showMyDialog() async {
                return showDialog<void>(
                  context: context, useSafeArea: true,
                  barrierDismissible: true, // user must tap button!
                  builder: (BuildContext context) {
                    return AlertDialog(
                        backgroundColor: Colors.black87,
                        title: const Text(
                          'Select Template to Load',
                          style: const TextStyle(
                              fontSize: 32, color: Colors.blueGrey),
                        ),
                        content: Container(
                          // color: ,
                          height: MediaQuery.of(context).size.height * 0.7,
                          width: MediaQuery.of(context).size.width * 0.7,
                          child: GridView(
                            gridDelegate:
                                SliverGridDelegateWithFixedCrossAxisCount(
                                    childAspectRatio: 1.25,
                                    crossAxisCount: 2,
                                    mainAxisSpacing: 50,
                                    crossAxisSpacing: 50),
                            children: [
                              ...templatesProvider.entries.map(
                                (entry) {
                                  return GestureDetector(
                                    onTap: () async {
                                      Future<void> writeToFile(
                                          ByteData data, String path) {
                                        final buffer = data.buffer;
                                        return new File(path).writeAsBytes(
                                            buffer.asUint8List(
                                                data.offsetInBytes,
                                                data.lengthInBytes));
                                      }

//read and write
                                      final filename =
                                          '${entry.value.item4}.json';
                                      var bytes = await rootBundle
                                          .load("assets/templates/$filename");
                                      String dir =
                                          (await getTemporaryDirectory()).path;
                                      writeToFile(bytes, '$dir/$filename')
                                          .then((value) {
                                        // print(dir);
                                        // print(filename);
                                        store
                                            .msgImport("$dir/$filename",
                                                timeout: Duration(seconds: 60))
                                            .then((value) =>
                                                Navigator.of(context).pop())
                                            .then((value) =>
                                                store.msgFitNodesToScreen(""));
                                      });
//write to app path
                                    },
                                    child: Padding(
                                      padding: const EdgeInsets.all(8.0),
                                      child: Container(
                                        decoration: BoxDecoration(
                                            color: Colors.blueGrey[200],
                                            borderRadius:
                                                BorderRadius.circular(5),
                                            border: Border.all(
                                                color: Colors.grey, width: 1)),
                                        child: Column(
                                          crossAxisAlignment:
                                              CrossAxisAlignment.start,
                                          children: [
                                            Padding(
                                              padding:
                                                  const EdgeInsets.all(8.0),
                                              child: Text(
                                                  "${entry.value.item1}",
                                                  style: const TextStyle(
                                                      fontSize: 24,
                                                      fontWeight:
                                                          FontWeight.bold)),
                                            ),
                                            Padding(
                                              padding:
                                                  const EdgeInsets.all(8.0),
                                              child: Text(
                                                  "${entry.value.item2}",
                                                  style: const TextStyle(
                                                      fontSize: 16,
                                                      fontWeight:
                                                          FontWeight.normal)),
                                            ),
                                            Padding(
                                              padding:
                                                  const EdgeInsets.all(8.0),
                                              child: Text(
                                                  "Featured nodes: ${entry.value.item3}",
                                                  style: const TextStyle(
                                                      fontSize: 16,
                                                      fontWeight:
                                                          FontWeight.normal)),
                                            ),
                                            Expanded(
                                              child: Container(
                                                  // height: 300,
                                                  // width: 200,
                                                  child: Image.asset(
                                                "assets/templates/${entry.value.item4}.png",
                                                //bundle: rootBundle,
                                              )),
                                            ),
                                          ],
                                        ),
                                      ),
                                    ),
                                  );
                                },
                              ).toList()
                            ],
                          ),
                        ));
                  },
                );
              }

              _showMyDialog();
            },
          ),
          TextButton.icon(
            style: TextButton.styleFrom(
              primary: Colors.blueGrey.shade400,
            ),
            icon: const Icon(Icons.help_outline_outlined),
            label: showButtonText ? const Text("help") : Text(""),
            onPressed: () {
              Future<void> _showMyDialog() async {
                return showDialog<void>(
                  context: context, useSafeArea: true,
                  barrierDismissible: true, // user must tap button!
                  builder: (BuildContext context) {
                    return AlertDialog(
                        backgroundColor: Colors.black87,
                        title: const Text(
                          'Help Section',
                          style: const TextStyle(
                              fontSize: 32, color: Colors.blueGrey),
                        ),
                        content: Container(
                          // color: ,
                          height: MediaQuery.of(context).size.height * 0.7,
                          width: MediaQuery.of(context).size.width * 0.7,
                          child: GridView(
                            gridDelegate:
                                SliverGridDelegateWithFixedCrossAxisCount(
                                    childAspectRatio: 1,
                                    crossAxisCount: 2,
                                    mainAxisSpacing: 50,
                                    crossAxisSpacing: 50),
                            children: [
                              ...helpProvider.entries.map(
                                (entry) {
                                  return Padding(
                                    padding: const EdgeInsets.all(8.0),
                                    child: Container(
                                      decoration: BoxDecoration(
                                          color: Colors.blueGrey[200],
                                          borderRadius:
                                              BorderRadius.circular(5),
                                          border: Border.all(
                                              color: Colors.grey, width: 1)),
                                      child: Column(
                                        crossAxisAlignment:
                                            CrossAxisAlignment.start,
                                        children: [
                                          Padding(
                                            padding: const EdgeInsets.all(8.0),
                                            child: Text("${entry.value.item1}",
                                                style: const TextStyle(
                                                    fontSize: 24,
                                                    fontWeight:
                                                        FontWeight.bold)),
                                          ),
                                          Padding(
                                            padding: const EdgeInsets.all(8.0),
                                            child: Text("${entry.value.item2}",
                                                style: const TextStyle(
                                                    fontSize: 16,
                                                    fontWeight:
                                                        FontWeight.normal)),
                                          ),
                                          Expanded(
                                            child: Container(
                                                // height: 300,
                                                // width: 200,
                                                child: Image.asset(
                                              "assets/help/${entry.value.item3}.gif",
                                              //bundle: rootBundle,
                                            )),
                                          ),
                                        ],
                                      ),
                                    ),
                                  );
                                },
                              ).toList()
                            ],
                          ),
                        ));
                  },
                );
              }

              _showMyDialog();
            },
          ),
        ],
      ),
    );
  }
}
