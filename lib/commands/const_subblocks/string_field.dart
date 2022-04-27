import 'dart:convert';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/commands/const.dart';
import 'package:moon/providers/focus_reject.dart';

import 'package:moon/providers/store_provider.dart';

class StringTextField extends HookConsumerWidget {
  StringTextField({Key? key, required this.treeNode, required this.focusNode})
      : super(key: key);

  final TreeNode treeNode;
  final FocusNode focusNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final focusNode = useFocusNode();
    final scrollController = useScrollController();

    var controller = useTextEditingController(
        text: treeNode.node.value.text != ""
            ? jsonDecode(treeNode.node.value.text)["Const"]["String"]
            : treeNode.node.value.text);
    final store = ref.read(storeRepoProvider).store;
    ValueNotifier<String> stringContent = useState("");

    saveToDb() {
      final text = stringContent.value.trimRight();

      final inputEvent = createJson(
        text,
        treeNode.node.key,
        "String",
      );
      // print(text);
      store.msgSendJson(inputEvent, timeout: Duration(minutes: 1));
    }

    useEffect(() {
      focusNode.addListener(() {
        if (focusNode.hasFocus) {
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
          saveToDb();

          ref.read(focusRejectController.notifier).set([]);
        }
      });

      return;
    }, [focusNode]);

    return Expanded(
      child: Container(
        child: SingleChildScrollView(
          child: Padding(
            padding: const EdgeInsets.fromLTRB(0, 0, 0, 0),
            child: Container(
              decoration: BoxDecoration(
                color: Color(0xFFEEEEEE),
                // border: Border.all(
                //   color: Color(0xFF258ED5),
                // ),
              ),
              width: treeNode.node.value.width - 120,
              // height: treeNode.node.value.height - 120,
              child: TextField(
                selectionControls: desktopTextSelectionControls,
                dragStartBehavior: DragStartBehavior.down, // expands: true,
                onTap: () {
                  // focusNode.requestFocus();
                },
                focusNode: focusNode,
                minLines: 8,
                maxLines: 8,
                keyboardType: TextInputType.multiline,
                textInputAction: TextInputAction.done,
                onEditingComplete: () {},
                scrollController: scrollController,

                onChanged: (value) {
                  stringContent.value = value;
                  // print(focusNode);

                  // final inputProperties = {
                  //   "node_id": "dummy",
                  //   "value": value,
                  // };
                  // String inputEvent =
                  //     JsonMapper.serialize(
                  //         InputProperties(
                  //             inputProperties));
                  // store.store.msgSetText(inputEvent);
                },
                onSubmitted: (_) {
                  // print(controller.value.text);
                  saveToDb();
                },
                controller: controller,
                obscureText: false,
                decoration: InputDecoration(
                  enabledBorder: const OutlineInputBorder(
                    borderSide: BorderSide(
                      color: Color.fromARGB(255, 255, 255, 255),
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
                // style: FlutterFlowTheme
                //     .bodyText1,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
