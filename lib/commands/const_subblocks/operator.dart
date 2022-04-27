import 'dart:convert';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/commands/const.dart';
import 'package:moon/providers/const_dropdown.dart';

import 'package:moon/providers/store_provider.dart';

class OperatorField extends HookConsumerWidget {
  OperatorField({Key? key, required this.treeNode, required this.focusNode})
      : super(key: key);

  final TreeNode treeNode;
  final FocusNode focusNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final store = ref.read(storeRepoProvider).store;
    final valueList = ref.read(operatorValues);

    final dropDownFocusNode = useFocusNode();

    var state = useState(treeNode.node.value.text != ""
        ? jsonDecode(treeNode.node.value.text)["Const"]["Operator"]
        : "Greater");

    saveToDb() {
      final text = state.value;

      final inputEvent = createJson(
        text,
        treeNode.node.key,
        "Operator",
      );

      store.msgSendJson(inputEvent, timeout: Duration(minutes: 1));
    }

    useEffect(() {
      if (treeNode.node.value.text == "") saveToDb();
      state.addListener(() {
        saveToDb();
      });
    }, [state]);

    // ValueNotifier<String> dropDownValue =
    //     treeNode.node.value.additionalData == ""
    //         ? useState("Greater")
    //         : useState(treeNode.node.value.additionalData);

    // final untrackedDropDownValue = treeNode.node.value.additionalData == ""
    //     ? "operator"
    //     : treeNode.node.value.additionalData;

    List<DropdownMenuItem<String>> dropDownList = valueList
        .map<String, DropdownMenuItem<String>>(
          (k, v) {
            return MapEntry(
              k,
              DropdownMenuItem(
                  child: Row(
                    children: [
                      Text("$k"),
                      Padding(
                        padding: const EdgeInsets.fromLTRB(10, 0, 0, 0),
                        child: Text(
                          "${v.item2}",
                          style: TextStyle(
                              fontWeight: FontWeight.bold, fontSize: 22),
                        ),
                      ),
                    ],
                  ),
                  value: v.item1),
            );
          },
        )
        .values
        .toList();

    return Container(
      width: treeNode.node.value.width - 120,
      height: 50,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(8, 8, 0, 0),
        child: DropdownButton(
          focusNode: dropDownFocusNode,
          isExpanded: true,
          items: dropDownList,
          onChanged: (value) {
            state.value = value.toString();
            Future.delayed(
                Duration(milliseconds: 30), () => dropDownFocusNode.unfocus());
          },
          value: state.value,
        ),
      ),
    );
  }
}
