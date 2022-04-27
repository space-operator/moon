import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart';
import 'package:recase/recase.dart';

import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class GuideManager extends HookConsumerWidget {
  const GuideManager({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final store = ref.read(storeRepoProvider).store;
    final selected_node_ids = ref.watch(selectedNodeIds);
    //update guide with new dropdown selection
    ref.watch(widgetTreeController);

    final scrollController = useScrollController();
    final textController = useTextEditingController();

    final text = useState("");
    useEffect(
      () {
        textController.addListener(() {
          text.value = textController.text;
        });
      },
    );

    List<String>? commandId;
    String? widgetType;
    String? additionalType;

    if (selected_node_ids.isNotEmpty) {
      commandId = ref.read(storeRepoProvider).selected_command_ids;
      if (commandId.isNotEmpty) {
        widgetType = store.view.nodes.entries
            .where((element) => element.key == commandId!.first)
            .first
            .value
            .widgetType
            .name;

        additionalType = store.view.nodes.entries
            .where((element) => element.key == commandId!.first)
            .first
            .value
            .additionalData;
      }
    }

    Widget? additionalConst;
    if (additionalType != null) {
      print(additionalType);
      switch (additionalType) {
        case "string":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "accepts any string",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "pubkey":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "accepts any valid pubkey",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "json":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "accepts any valid JSON.",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "file_picker":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "select a file to pass it's absolute path an output",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "bool_true":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "passed 'bool = true' to the output",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "bool_false":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "passed 'bool = false' to the output",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "operator":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "select operator to pass to the Branch command",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "seed":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                SelectableText(
                  "generate or paste a 12 word seed phrase.\nHere is a useful link:\nhttps://iancoleman.io/bip39/",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "i64":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "64-bit signed integer\nmin:-9223372036854775808\nmax: 9223372036854775808",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "u8":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "8-bit unsigned integer\nmin: 0\nmax: 255",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "u16":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "16-bit unsigned integer\nmin: 0\nmax: 65535",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "u64":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "64-bit unsigned integer\nmin: 0\nmax: 18446744073709551615",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "f32":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "A 32-bit floating point type\n'can represent a wide range of decimal numbers, like 3.5, 27, -113.75, 0.0078125, 34359738368, 0, -1'",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "f64":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "A 64-bit floating point type\n'can represent a wide range of decimal numbers, like 3.5, 27, -113.75, 0.0078125, 34359738368, 0, -1'",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        case "custom":
          {
            additionalConst = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "Selected type: ${additionalType.titleCase}",
                  style: const TextStyle(color: Colors.amber),
                ),
                Text(
                  "A JSON in the following format\n{\"CustomTypeName\":\n      {place JSON content here}\n}\n\nFor type 'nft metadata' enter 'NftMetadata' as the custom type.\n\ne.g. Create Metadata Accounts, Uses input: \n{\n\"Uses\":{\n            \"use_method\": \"Burn\",\n            \"remaining\": 500,\n            \"total\": 500\n        }\n}\n",
                  style: TextStyle(color: Colors.lightBlue.shade300),
                ),
              ],
            );
          }
          break;
        default:
          {
            additionalConst = const Text("");
          }
      }
    }

    /// get list of commands
    ///

    final commands = ref.read(storeRepoProvider).text_commands;

    final commandList = commands.where(
      (element) {
        if (commandId != null) {
          final ReCase rc = ReCase(element.commandName);

          return rc.pascalCase == widgetType;
        } else {
          return element.commandName.titleCase
              .toLowerCase()
              .contains(text.value.toLowerCase());
          // return true;
        }
      },
    ).map(
      (widgetTextCommand) {
        final ReCase rc = ReCase(widgetTextCommand.commandName);

        return Container(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: Text(rc.titleCase,
                    style: TextStyle(
                        color: Colors.amber.shade300,
                        fontWeight: FontWeight.bold,
                        fontSize: 18)),
              ),
              Padding(
                padding: const EdgeInsets.fromLTRB(10, 0, 0, 0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    if (widgetTextCommand.availability.length < 3)
                      Padding(
                        padding: const EdgeInsets.fromLTRB(0, 0, 8, 8),
                        child: Text(
                            "Only supported on ${widgetTextCommand.availability.map(
                                  (e) => e.titleCase,
                                ).join(", ")}",
                            style: TextStyle(
                                // fontStyle: FontStyle,
                                color: Colors.amber[600])),
                      ),
                    SelectableText(widgetTextCommand.description,
                        style: const TextStyle(color: Colors.white)),
                    if (widgetTextCommand.inputs.length > 0) ...[
                      const Divider(),
                      Padding(
                        padding: const EdgeInsets.fromLTRB(0, 0, 8, 8),
                        child: Text("INPUTS",
                            style: TextStyle(color: Colors.amber.shade300)),
                      ),
                      ...widgetTextCommand.inputs.map(
                        (e) {
                          final ReCase rc = ReCase(e.name);
                          final required = e.required ? "required" : "optional";
                          return Padding(
                            padding: const EdgeInsets.fromLTRB(8, 0, 8, 0),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Padding(
                                  padding:
                                      const EdgeInsets.fromLTRB(0, 0, 8, 5),
                                  child: Row(
                                    children: [
                                      Text("${rc.titleCase} ($required)",
                                          style: const TextStyle(
                                              color: Colors.white)),
                                    ],
                                  ),
                                ),
                                if (e.tooltip != "")
                                  Padding(
                                    padding:
                                        const EdgeInsets.fromLTRB(8, 0, 8, 5),
                                    child: SelectableText("${e.tooltip}",
                                        style: TextStyle(
                                            color: Colors.lightBlue.shade50)),
                                  ),
                                Padding(
                                  padding:
                                      const EdgeInsets.fromLTRB(8, 0, 8, 0),
                                  child: Column(
                                    crossAxisAlignment:
                                        CrossAxisAlignment.start,
                                    children: [
                                      if (e.hasDefault)
                                        Padding(
                                          padding: const EdgeInsets.fromLTRB(
                                              0, 8, 0, 8),
                                          child: Text(
                                              "default value: ${e.defaultValue}",
                                              style: const TextStyle(
                                                  // fontStyle: FontStyle,
                                                  color: Colors.white)),
                                        ),
                                      Text("accepted types:",
                                          style: TextStyle(
                                              // fontStyle: FontStyle,
                                              color:
                                                  Colors.lightBlue.shade300)),
                                      Padding(
                                        padding: const EdgeInsets.all(8.0),
                                        child: Text(
                                            "${e.acceptableKinds.map((e) => e.titleCase).join(", ").toLowerCase()}",
                                            style: TextStyle(
                                                fontStyle: FontStyle.italic,
                                                color:
                                                    Colors.lightBlue.shade300),
                                            softWrap: true),
                                      ),
                                    ],
                                  ),
                                ),
                                Padding(
                                  padding:
                                      const EdgeInsets.fromLTRB(0, 8, 8, 8),
                                  child: Divider(color: Colors.blueGrey[300]),
                                ),
                              ],
                            ),
                          );
                        },
                      ),
                    ],
                    if (widgetTextCommand.outputs.length > 0) ...[
                      const Divider(),
                      Padding(
                        padding: const EdgeInsets.fromLTRB(0, 0, 8, 8),
                        child: Text("OUTPUTS",
                            style: TextStyle(color: Colors.amber.shade300)),
                      ),
                      ...widgetTextCommand.outputs.map(
                        (e) {
                          final ReCase rc = ReCase(e.name);
                          final passthrough =
                              e.passthrough ? "(passthrough field)" : "";
                          return Padding(
                            padding: const EdgeInsets.fromLTRB(8, 0, 8, 0),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Padding(
                                  padding:
                                      const EdgeInsets.fromLTRB(0, 0, 8, 5),
                                  child: Row(
                                    children: [
                                      Text(
                                        "${rc.titleCase}\n $passthrough",
                                        style: TextStyle(
                                            color: Colors.lightBlue.shade50),
                                      ),
                                    ],
                                  ),
                                ),
                                if (e.tooltip != "")
                                  Padding(
                                    padding:
                                        const EdgeInsets.fromLTRB(8, 0, 8, 5),
                                    child: SelectableText("${e.tooltip}",
                                        style: TextStyle(
                                            color: Colors.lightBlue.shade50)),
                                  ),
                                if (e.kind != "")
                                  Padding(
                                    padding:
                                        const EdgeInsets.fromLTRB(8, 0, 8, 0),
                                    child: Column(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.start,
                                      children: [
                                        Text("output type:",
                                            style: TextStyle(
                                                // fontStyle: FontStyle,
                                                color:
                                                    Colors.lightBlue.shade300)),
                                        Padding(
                                          padding: const EdgeInsets.all(8.0),
                                          child: Text("${e.kind.toLowerCase()}",
                                              style: TextStyle(
                                                  fontStyle: FontStyle.italic,
                                                  color: Colors
                                                      .lightBlue.shade300),
                                              softWrap: true),
                                        ),
                                      ],
                                    ),
                                  ),
                                Padding(
                                  padding:
                                      const EdgeInsets.fromLTRB(0, 8, 8, 8),
                                  child: Divider(color: Colors.blueGrey[300]),
                                ),
                              ],
                            ),
                          );
                        },
                      ),
                      if (additionalType != null) additionalConst!
                    ],
                    // Draggable(
                    //   onDragEnd: (details) {
                    //     // print(details.offset);
                    //   },
                    //   child: Image.asset(
                    //     "assets/const.png",
                    //     // height: 100,
                    //   ),
                    //   feedback: Image.asset(
                    //     "assets/const.png",
                    //     // height: 100,
                    //   ),
                    // )
                  ],
                ),
              )
            ],
          ),
        );
      },
    ).toList();

    commands.sort(((a, b) {
      return a.commandName.compareTo(b.commandName);
    }));

    return Expanded(
      child: Container(
        width: 270,
        decoration: BoxDecoration(color: Colors.blueGrey[800]),
        child: Column(
          children: [
            Container(
              width: 270,
              height: 35,
              color: Colors.blueGrey,
              child: Padding(
                  padding: EdgeInsets.all(8),
                  child: const Text(
                    "Guide",
                    style: TextStyle(color: Colors.amberAccent, fontSize: 14),
                  )),
            ),
            if (commandId == null)
              Container(
                height: 60,
                color: Colors.blueGrey,
                child: Row(
                  children: [
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: const Icon(Icons.search),
                    ),
                    Expanded(
                      child: Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: TextField(
                          decoration:
                              InputDecoration(hintText: "enter command name"),
                          controller: textController,
                          // onChanged: (value) {
                          //   textController.text = value);
                          // },
                        ),
                      ),
                    ),
                    if (text.value != "")
                      IconButton(
                        icon: const Icon(Icons.clear),
                        onPressed: () {
                          textController.clear();
                        },
                      )
                  ],
                ),
              ),
            Expanded(
              child: Container(
                width: 270,
                decoration: BoxDecoration(color: Colors.blueGrey[800]),
                child: ListView.separated(
                  controller: scrollController,
                  separatorBuilder: ((context, index) => const Divider()),
                  itemBuilder: ((context, index) {
                    return commandList[index];
                  }),
                  itemCount: commandList.length,
                  // children: commandList,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
