import 'package:dart_json_mapper/dart_json_mapper.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:flutter/material.dart';
import 'package:recase/recase.dart';
import 'package:moon/utils/logger.dart';
import 'package:moon/providers/popup_menu.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:moon/serialization/input_mapping.dart';
import 'package:moon/widgets/block.dart';

/// Must call ApplyCommand in two places
///
/// 1. onFieldSubmitted; where user types
/// 2. InkWell, onTap; where user clicks on autocomplete selection
///
///
class TextInput extends SuperBlock {
  TextInput(
      {Key? key,
      BuildContext? context,
      required this.treeNode,
      required this.parentId})
      : super(key: key);

  final TreeNode treeNode;
  final String parentId;
  // final FocusNode selectedNode;

  // final FocusNode focusNode;
  // final TextEditingController _controller;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final selectedIds = ref.watch(selectedNodeIds);
    final selected = selectedIds.contains(parentId);

    // final FocusNode focusNode = useFocusNode();
    final nodes = ref.watch(nodeController);

    final store = ref.read(storeRepoProvider).store;
    List<rid.WidgetTextCommand> _userOptions = store.view.textCommands;
    _userOptions.sort((a, b) {
      return a.commandName.toLowerCase().compareTo(b.commandName.toLowerCase());
    });

    String _displayStringForOption(rid.WidgetTextCommand option) {
      final avail = option.availability.length < 3
          ? "(${option.availability.map(
                (e) => e.titleCase,
              ).join(", ")})"
          : "";

      final ReCase rc = ReCase(option.commandName);

      return rc.titleCase + " " + avail; //
    }

    // final TextEditingController _controller =
    //     useTextEditingController(text: node.value.text);

    final double optionsMaxHeight = 200;
    final double optionsMaxWidth = treeNode.node.value.width.toDouble();

    return
        // ProviderScope(
        //   overrides: [currentNode.overrideWithValue(treeNode)],
        //   child:
        Positioned(
      height: treeNode.node.value.height.toDouble(),
      width: treeNode.node.value.width.toDouble(),
      left: treeNode.node.value.x.toDouble(),
      top: treeNode.node.value.y.toDouble(),
      child: Card(
        shape: ref.read(selectedNode(selected)),
        child: Stack(
          children: [
            // Text(treeNode.node.key), // to debug node id
            Padding(
              padding: const EdgeInsets.fromLTRB(8, 8, 30, 8),
              child: Autocomplete(
                displayStringForOption: _displayStringForOption,
                fieldViewBuilder: (BuildContext context,
                    TextEditingController textEditingController,
                    FocusNode focusNode,
                    VoidCallback onFieldSubmitted) {
                  textEditingController.value =
                      TextEditingValue(text: treeNode.node.value.text);

                  return TextFormField(
                    decoration: const InputDecoration(
                      hintText: 'press / for commands',
                    ),
                    controller:
                        textEditingController, // check controller and focus
                    focusNode: focusNode,
                    autofocus: true,
                    onTap: () {
                      textEditingController.selection =
                          TextSelection.fromPosition(TextPosition(
                              offset: treeNode.node.value.text.length));
                    },
                    onChanged: (text) {
                      // textEditingController.value = TextEditingValue(text: text);
                      // print(text);
                    },
                    onEditingComplete: () {},
                    onFieldSubmitted: (String value) {
                      onFieldSubmitted();
                      final commandNameReCase = textEditingController.text;
                      // what user typed
                      // print(value);
                      //converted command to remove mainnet, devnet text
                      final commandValue = commandNameReCase.split(" (").first;
                      // print(commandValue);
                      final ReCase rc = ReCase(commandValue);
                      // convert back to snake_case since options have been recased
                      final commandName = rc.snakeCase;
                      log.v(commandName);

                      // prevent non-existent command from being called
                      final match = _userOptions.where(((textCommand) {
                        return textCommand.commandName == commandName;
                      }));
                      if (match.isNotEmpty &&
                          commandName == match.first.commandName) {
                        store.msgApplyCommand(parentId, commandName,
                            timeout: Duration(minutes: 1)); // call ApplyCommand
                        focusNode.unfocus();
                      }

                      if (match.isEmpty) {
                        final text = textEditingController.value.text;
                        log.v(text);
                        final inputProperties = {
                          "nodeId": treeNode.node.key,
                          "text": text
                        };
                        final String inputEvent = JsonMapper.serialize(
                            InputProperties(inputProperties));
                        store.msgSetText(inputEvent,
                            timeout: Duration(minutes: 1));
                      }
                    },
                  );
                },
                optionsBuilder: ((textEditingValue) {
                  if (textEditingValue.text == '') {
                    return const Iterable<rid.WidgetTextCommand>.empty();
                  }
                  if (textEditingValue.text.startsWith('/')) {
                    // remove slash and pass to options
                    final newTextEditingValue = textEditingValue.replaced(
                        TextRange(start: 0, end: 1), "");

                    // recast to snake case to catch underscore
                    final ReCase rc = ReCase(newTextEditingValue.text);

                    return _userOptions.where((rid.WidgetTextCommand option) {
                      return option.toString().contains(rc.snakeCase);
                    });
                  } else {
                    return const Iterable<rid.WidgetTextCommand>.empty();
                  }
                }),
                optionsViewBuilder: (
                  BuildContext context,
                  AutocompleteOnSelected<rid.WidgetTextCommand> onSelected,
                  Iterable<rid.WidgetTextCommand> options,
                ) {
                  return Align(
                    alignment: Alignment.topLeft,
                    child: Material(
                      elevation: 4.0,
                      child: ConstrainedBox(
                        constraints: BoxConstraints(
                            maxHeight: optionsMaxHeight,
                            maxWidth: optionsMaxWidth),
                        child: ListView.builder(
                          padding: EdgeInsets.zero,
                          shrinkWrap: true,
                          itemCount: options.length,
                          itemBuilder: (BuildContext context, int index) {
                            final rid.WidgetTextCommand option =
                                options.elementAt(index);
                            return InkWell(
                              onTap: () {
                                onSelected(option);
                                store.msgApplyCommand(
                                    parentId, option.commandName,
                                    timeout: Duration(
                                        minutes: 1)); // call ApplyCommand
                              },
                              child: Builder(
                                builder: (BuildContext context) {
                                  final bool highlight =
                                      AutocompleteHighlightedOption.of(
                                              context) ==
                                          index;
                                  if (highlight) {
                                    SchedulerBinding.instance!
                                        .addPostFrameCallback(
                                      (Duration timeStamp) {
                                        Scrollable.ensureVisible(context,
                                            alignment: 0.5);
                                      },
                                    );
                                  }
                                  return Container(
                                    color: highlight
                                        ? Theme.of(context).focusColor
                                        : null,
                                    padding: const EdgeInsets.all(16.0),
                                    child:
                                        Text(_displayStringForOption(option)),
                                  );
                                },
                              ),
                            );
                          },
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),
            Positioned(
              right: 0,
              child: ref.read(
                popUpMenuProvider(parentId),
              ),
            ),
          ],
        ),
      ),
      // ),
    );
  }
}

/*
TextField(
            autofocus: false,
            focusNode: focusNode,
            controller: _controller,
            onChanged: (value) {
              //disposing focusnode while being used
            },
            onSubmitted: (value) {
              final inputProperties = {
                "nodeId": node.key,
                "text": value,
              };
              String inputEvent =
                  JsonMapper.serialize(InputProperties(inputProperties));
              store.store.msgSetText(inputEvent);
            },
            onTap: () {
              focusNode.requestFocus();
              // FocusNodeManager.instance.removeFocus(context);
              // FocusNodeManager.instance.requestFocus(context, selectedNode);
            },

            // onEditingComplete: () =>
            //     Focus.of(context).ancestors.first.requestFocus(),
          ),
 */
