import 'dart:convert';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/commands/const.dart';
import 'package:moon/providers/focus_reject.dart';

import 'package:moon/providers/store_provider.dart';

class NumberTextField extends HookConsumerWidget {
  NumberTextField({
    Key? key,
    required this.treeNode,
    required this.numberType,
    required this.numberIs,
  }) : super(key: key);

  final TreeNode treeNode;
  final String numberType;
  final Type numberIs;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final focusNode = useFocusNode();
    var controller = useTextEditingController(
        text: treeNode.node.value.text == ""
            ? ""
            : jsonDecode(treeNode.node.value.text)["Const"][numberType]
                .toString());
    final store = ref.read(storeRepoProvider).store;

    // final ReCase rc = ReCase(numberType);
    ValueNotifier<String> _error = useState("");
    ValueNotifier<bool> decodeSucceeded = useState(false);

    void saveToDb() {
      if (decodeSucceeded.value) {
        final inputEvent = createJson(
          numberIs == int
              ? int.parse(controller.value.text)
              : double.parse(controller.value.text),
          treeNode.node.key,
          numberType.toString().toUpperCase(),
        );
        store.msgSendJson(inputEvent, timeout: Duration(minutes: 1));
      }
    }

    useEffect(() {
      focusNode.addListener(() {
        if (focusNode.hasFocus) {
          final focusRect = focusNode.rect;

          final rect = Rect.fromLTRB(
            focusRect.left - 30,
            focusRect.top,
            focusRect.right + 15,
            focusRect.top + treeNode.node.value.height - 120 + 30,
          );
          ref.read(focusRejectController.notifier).set([rect]);
        } else {
          saveToDb();

          ref.read(focusRejectController.notifier).set([]);
        }
      });

      return;
    }, [focusNode]);

    return Container(
      child: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.fromLTRB(0, 0, 0, 0),
          child: Container(
            decoration: BoxDecoration(
              color: Color(0xFFEEEEEE),
              border: Border.all(
                color: Color(0xFF258ED5),
              ),
            ),
            width: treeNode.node.value.width - 120,
            height: treeNode.node.value.height - 120,
            child: TextField(
              selectionControls: desktopTextSelectionControls,
              dragStartBehavior: DragStartBehavior.down,
              onTap: () {},
              focusNode: focusNode,
              minLines: 1,
              maxLines: 1,
              keyboardType: TextInputType.number,
              textInputAction: TextInputAction.done,
              onEditingComplete: () {},
              onChanged: (value) {
                _error.value = "";
                decodeSucceeded.value = false;

                if (numberIs == int) {
                  final number = int.tryParse(value);
                  if (number == null) {
                    _error.value = 'Must be a integer';
                    decodeSucceeded.value = false;
                  } else {
                    decodeSucceeded.value = true;
                  }
                }
                if (numberIs == double) {
                  final float = double.tryParse(value);
                  if (float == null) {
                    decodeSucceeded.value = false;
                    _error.value = 'Must be a float';
                  } else {
                    decodeSucceeded.value = true;
                  }
                }
                if (value.isEmpty) {
                  _error.value = "";
                }
              },
              onSubmitted: (_) {
                saveToDb();
              },
              controller: controller,
              obscureText: false,
              decoration: InputDecoration(
                focusedErrorBorder: InputBorder.none,
                errorBorder: InputBorder.none,
                errorText: _error.value,
                border: const OutlineInputBorder(),
                enabledBorder: const UnderlineInputBorder(
                  borderSide: const BorderSide(
                    color: Color(0x00000000),
                    width: 1,
                  ),
                  borderRadius: const BorderRadius.only(
                    topLeft: Radius.circular(2.0),
                    topRight: Radius.circular(2.0),
                  ),
                ),
                disabledBorder: InputBorder.none,
                focusedBorder: const UnderlineInputBorder(
                  borderSide: BorderSide(
                    color: Color(0x00000000),
                    width: 2,
                  ),
                  borderRadius: const BorderRadius.only(
                    topLeft: const Radius.circular(2.0),
                    topRight: const Radius.circular(2.0),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
