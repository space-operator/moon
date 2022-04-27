import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/nodes/add_port.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:tuple/tuple.dart';
import 'package:recase/recase.dart';

class BasicPort extends HookConsumerWidget {
  const BasicPort(
    this.port_type,
    this.nodeEntry,
    this.commandName, {
    Key? key,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final PortType port_type;
  final String commandName;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding port entry");
    final ReCase rc = ReCase(nodeEntry.item2.text);
    final String typeBounds;
    switch (commandName) {
      case "Const":
      case "Wait":
      case "Print":
        {
          typeBounds = "";
        }
        break;

      default:
        typeBounds = nodeEntry.item2.typeBounds.titleCase.toLowerCase();
    }

    return port_type == PortType.input
        ? Expanded(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(5, 0, 0, 0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    "${rc.sentenceCase}", //${inputName}",
                    style: TextStyle(color: Colors.blue[700]),
                    maxLines: 2,
                    softWrap: true,
                  ),
                  Text(
                    !nodeEntry.item2.hasDefault
                        ? "${typeBounds}"
                        : "${typeBounds} (default:${nodeEntry.item2.defaultValue})", //${inputName}",
                    style: TextStyle(
                      fontSize: 9,
                      fontStyle: FontStyle.italic,
                    ),
                    textAlign: TextAlign.start,
                  ),
                ],
              ),
            ),
          )
        : Expanded(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(0, 0, 5, 0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.end,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    "${rc.sentenceCase}", //${inputName}",
                    style: TextStyle(
                        color: nodeEntry.item2.passthrough
                            ? Colors.blue[700]
                            : Colors.black),
                    textAlign: TextAlign.end,
                    maxLines: 2,
                    softWrap: true,
                  ),
                  Text(
                    "${typeBounds}",
                    style: const TextStyle(
                        fontSize: 9, fontStyle: FontStyle.italic),
                    textAlign: TextAlign.end,
                  ),
                ],
              ),
            ),
          );
  }
}

// HookConsumerWidget PortEntry(
//   port_type,
//   // inputType,
//   Tuple2<String, rid.NodeView> nodeEntry,
//   String commandName,
// ) {
//   HookConsumerWidget? _widget;

//   switch (nodeEntry.item2.widgetType) {
//     // case "String":
//     //   {
//     //     _widget = TextEntry(port_type, inputType, nodeEntry);
//     //   }
//     //   break;
//     // case "String":

//     // case "Number":
//     // case "wait":
//     // case "Pubkey":
//     //   {
//     //     _widget = BasicPort(port_type, inputType, nodeEntry);
//     //   }
//     //   break;

//     default:
//       {
//         _widget = BasicPort(
//           port_type,
//           nodeEntry,
//           commandName,
//           key: ObjectKey(nodeEntry),
//         );
//       }
//     // print(inputType);
//   }

//   return _widget;
// }

// class TextEntry extends HookConsumerWidget {
//   const TextEntry(
//     this.port_type,
//     this.inputType,
//     this.nodeEntry, {
//     Key? key,
//   }) : super(key: key);

//   final Tuple2<String, rid.NodeView> nodeEntry;
//   final String inputType;
//   final PortType port_type;

//   @override
//   Widget build(BuildContext context, WidgetRef ref) {
//     // final itemFocusNode = useFocusNode();
//     // // listen to focus chances
//     // useListenable(itemFocusNode);
//     // final isFocused = itemFocusNode.hasFocus;

//     final textEditingController = useTextEditingController();

//     final textFieldFocusNode = useFocusNode();
//     final ReCase rc = ReCase(nodeEntry.item2.text);

//     return port_type == PortType.input
//         ? Expanded(
//             child: Padding(
//                 padding: const EdgeInsets.fromLTRB(5, 0, 0, 0),
//                 child: TextField(
//                   focusNode: textFieldFocusNode,
//                   // autofocus: true,
//                   decoration: InputDecoration(
//                     focusedBorder: UnderlineInputBorder(),
//                     enabledBorder: InputBorder.none,
//                     labelText: rc.sentenceCase,
//                   ),
//                   controller: textEditingController,
//                   // style: TextStyle(),
//                   maxLines: 1,
//                 )),
//           )
//         : Expanded(
//             child: Padding(
//               padding: const EdgeInsets.fromLTRB(0, 0, 5, 0),
//               child: Text(
//                 "${rc.sentenceCase}", //${inputName}",
//                 // style: TextStyle(),
//                 textAlign: TextAlign.end,
//                 maxLines: 2,
//                 softWrap: true,
//               ),
//             ),
//           );
//   }
// }

/*
class TextEntry extends HookConsumerWidget {
  TextEntry(
    this.inputType,
    this.nodeEntry, {
    Key? key,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final String inputType;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final itemFocusNode = useFocusNode();
    // listen to focus chances
    useListenable(itemFocusNode);
    final isFocused = itemFocusNode.hasFocus;

    final textEditingController = useTextEditingController();

    final textFieldFocusNode = useFocusNode();

    return Expanded(
      child: Padding(
          padding: const EdgeInsets.fromLTRB(5, 0, 0, 0),
          child: Focus(
            focusNode: itemFocusNode,
            onFocusChange: (focused) {
              if (focused) {
                // textEditingController.text = todo.description;
              } else {
                // Commit changes only when the textfield is unfocused, for performance
                // ref
                //     .read(todoListProvider.notifier)
                //     .edit(id: todo.id, description: textEditingController.text);
              }
            },
            child: ListTile(
              onTap: () {
                itemFocusNode.requestFocus();
                textFieldFocusNode.requestFocus();
                print(itemFocusNode);
                print(textFieldFocusNode);
                print(FocusManager.instance.primaryFocus);
              },
              title: isFocused
                  ? TextField(
                      focusNode: textFieldFocusNode,
                      autofocus: true,
                      // decoration: InputDecoration(
                      //   focusedBorder: UnderlineInputBorder(),
                      //   enabledBorder: InputBorder.none,
                      //   labelText: nodeEntry.item2.text,
                      // ),
                      controller: textEditingController,
                      style: TextStyle(),
                      maxLines: 1,
                    )
                  : Text(nodeEntry.item2.text),
            ),
          )),
    );
  }
}
 */
