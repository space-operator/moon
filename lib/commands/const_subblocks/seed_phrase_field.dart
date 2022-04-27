import 'dart:convert';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/commands/const.dart';
import 'package:moon/providers/focus_reject.dart';
import 'package:moon/utils/logger.dart';
import 'package:tuple/tuple.dart';
import 'package:flutter/services.dart';

import 'package:moon/providers/store_provider.dart';

class SeedTextField extends HookConsumerWidget {
  SeedTextField({Key? key, required this.treeNode}) : super(key: key);

  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    log.d(
        "rebuilding Seed ${treeNode.node.value.widgetType} ${treeNode.node.key}");
    Future<void> _copyToClipboard(text) async {
      await Clipboard.setData(ClipboardData(text: text));
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: Text('Copied to clipboard', textAlign: TextAlign.center),
      ));
      // Scaffold.of(context).showSnackBar(snackbar)
    }

    final focusNode = useFocusNode();
    // final scrollController = useScrollController();
    // print(treeNode.node.value.text);
    var controller = useTextEditingController(
        text: treeNode.node.value.text != ""
            ? jsonDecode(treeNode.node.value.text)["Const"]["String"]
            : treeNode.node.value.text);
    final store = ref.read(storeRepoProvider).store;

    ValueNotifier<Tuple2<String, int?>> _error = useState(Tuple2("", null));
    ValueNotifier<bool> decodeSucceeded = useState(false);
    ValueNotifier<bool> obscureText = useState(true);

    saveToDb() {
      final text = controller.value.text.trimRight();

      final inputEvent = createJson(
        text,
        treeNode.node.key,
        "String",
      );
      store.msgSendJson(inputEvent, timeout: Duration(minutes: 1));
    }

    useEffect(() {
      focusNode.addListener(() {
        if (focusNode.hasFocus) {
          obscureText.value = false;
          // print("Has focus:${focusNode.hasFocus}");
          final focusRect = focusNode.rect;
          // print(focusNode.size);
          // print(focusRect.right);
          final rect = Rect.fromLTRB(
            focusRect.left - 30,
            focusRect.top,
            focusRect.right + 15,
            focusRect.top + treeNode.node.value.height - 120 + 30,
          );
          // print(rect.size);
          ref.read(focusRejectController.notifier).set([rect]);
        } else {
          obscureText.value = true;

          if (decodeSucceeded.value == true) saveToDb();

          ref.read(focusRejectController.notifier).set([]);
        }
      });

      return;
    }, [focusNode]);

    useEffect(() {
      controller.addListener(() {
        _error.value = Tuple2("", null);
        decodeSucceeded.value = false;

        final String s = controller.value.text;
        final RegExp regExp = new RegExp(r"[\w-._]+");
        final Iterable matches = regExp.allMatches(s);
        final int _count = matches.length;

        if (_count == 12) {
          decodeSucceeded.value = true;
          saveToDb();
        } else {
          _error.value = Tuple2(
              "Not a valid Seed, should have 12 words, not $_count", _count);
        }
      });
    }, [controller]);

    return Expanded(
      child: Container(
        width: treeNode.node.value.width - 120,
        height: 100,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(0, 0, 0, 0),
          child: Column(
            children: [
              Container(
                decoration: BoxDecoration(
                  color: Color(0xFFEEEEEE),
                ),
                child: TextField(
                  selectionControls: desktopTextSelectionControls,
                  dragStartBehavior: DragStartBehavior.down, // expands: true,
                  onTap: () {
                    // focusNode.requestFocus();
                  },
                  focusNode: focusNode,
                  minLines: 1,
                  maxLines: 1,
                  keyboardType: TextInputType.text,
                  textInputAction: TextInputAction.done,
                  onEditingComplete: () {},
                  onChanged: (value) {},

                  onSubmitted: (_) {
                    if (decodeSucceeded.value == true) saveToDb();
                  },
                  controller: controller,
                  obscureText: obscureText.value,
                  decoration: InputDecoration(
                    enabledBorder: OutlineInputBorder(
                      borderSide: BorderSide(
                        color: Color.fromARGB(255, 255, 255, 255),
                        width: 1,
                      ),
                    ),
                    errorText:
                        _error.value.item1 == "" ? null : _error.value.item1,
                    focusedErrorBorder: OutlineInputBorder(
                      borderSide: BorderSide(
                        color: Color.fromARGB(255, 255, 0, 0),
                        width: 1,
                      ),
                    ),
                    errorBorder: OutlineInputBorder(
                      borderSide: BorderSide(
                        color: Color.fromARGB(255, 255, 0, 0),
                        width: 1,
                      ),
                    ),
                    disabledBorder: InputBorder.none,
                    focusedBorder: OutlineInputBorder(
                      borderSide: BorderSide(
                        color: Colors.blue.shade300,
                        width: 1,
                      ),
                    ),
                  ),
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: Container(
                  height: 20,
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      TextButton.icon(
                        onPressed: () {
                          ref
                              .read(storeRepoProvider)
                              .store
                              .msgGenerateSeedPhrase("")
                              .then((value) => controller.value =
                                  TextEditingValue(text: value.data!));
                        },
                        icon: const Icon(Icons.refresh_outlined,
                            color: Colors.blueGrey),
                        label: Text(
                          "Generate new seed",
                          style:
                              TextStyle(fontSize: 12, color: Colors.blueGrey),
                        ),
                      ),
                      Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: Divider(color: Colors.white),
                      ),
                      TextButton.icon(
                        icon: const Icon(Icons.copy, color: Colors.blueGrey),
                        onPressed: () {
                          _copyToClipboard(controller.value.text);
                        },
                        label: Text(
                          "Copy",
                          style:
                              TextStyle(fontSize: 12, color: Colors.blueGrey),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
