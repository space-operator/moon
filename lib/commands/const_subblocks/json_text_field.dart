import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:recase/recase.dart';
import 'package:moon/commands/const.dart';
import 'package:moon/providers/focus_reject.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:tuple/tuple.dart';

class JsonTextField extends HookConsumerWidget {
  JsonTextField({Key? key, required this.treeNode}) : super(key: key);

  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final focusNode = useFocusNode();
    final scrollController = useScrollController();
    var controller = useTextEditingController(
        text: treeNode.node.value.text == ""
            ? ""
            : jsonEncode(
                jsonDecode(treeNode.node.value.text)["Const"]["Json"]));
    final store = ref.read(storeRepoProvider).store;

    ValueNotifier<Tuple2<String, int?>> _error = useState(Tuple2("", null));
    ValueNotifier<bool> decodeSucceeded = useState(false);
    ValueNotifier<Map<String, dynamic>> decodedJson = useState({});

    void saveToDb() {
      if (decodeSucceeded.value == true) {
        final text = decodedJson.value;
        // final output =
        //     createJson<Map<String, dynamic>>(text, treeNode.node.key, null);
        final output = createJson(
          text,
          treeNode.node.key,
          "Json",
        );
        // print(output);
        store.msgSendJson(output, timeout: Duration(minutes: 1));
      }
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
        height: treeNode.node.value.height - 120,
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
                dragStartBehavior: DragStartBehavior.down,
                // expands: true,
                onTap: () {
                  // focusNode.requestFocus();
                },
                focusNode: focusNode,
                minLines: 17,
                maxLines: 17,
                keyboardType: TextInputType.multiline,
                textInputAction: TextInputAction.done,
                onEditingComplete: () {},
                scrollController: scrollController,

                onChanged: (value) {
                  _error.value = Tuple2("", null);
                  decodeSucceeded.value = false;
                  try {
                    decodedJson.value =
                        json.decode(value) as Map<String, dynamic>;
                    decodeSucceeded.value = true;
                  } on FormatException catch (e) {
                    // print(e); //actually prints where the json error is

                    _error.value = Tuple2(
                        "Not a valid JSON, check cursor position ${e.offset}",
                        e.offset);
                  }
                },
                onSubmitted: (_) {
                  saveToDb();
                },
                controller: controller,
                decoration: InputDecoration(
                  enabledBorder: const OutlineInputBorder(
                    borderSide: BorderSide(
                      color: Color.fromARGB(255, 255, 255, 255),
                      width: 1,
                    ),
                  ),
                  errorText:
                      _error.value.item1 == "" ? null : _error.value.item1,
                  focusedErrorBorder: const OutlineInputBorder(
                    borderSide: BorderSide(
                      color: Color.fromARGB(255, 255, 0, 0),
                      width: 1,
                    ),
                  ),
                  errorBorder: const OutlineInputBorder(
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
