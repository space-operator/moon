import 'package:flutter/material.dart';

import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/commands/const.dart';

import 'package:moon/providers/store_provider.dart';

class BoolField extends HookConsumerWidget {
  BoolField({
    Key? key,
    required this.treeNode,
    required this.focusNode,
    required this.boolValue,
  }) : super(key: key);

  final TreeNode treeNode;
  final FocusNode focusNode;
  bool boolValue;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // for bool, automatically set the value to true/false
    useEffect(() {
      final value = createJson(
        boolValue,
        treeNode.node.key,
        "Bool",
      );
      // print(value);
      ref
          .read(storeRepoProvider)
          .store
          .msgSendJson(value, timeout: Duration(minutes: 1));
    });
    // final focusNode = useFocusNode();
    // final scrollController = useScrollController();
    // var controller = useTextEditingController(text: treeNode.node.value.text);
    // final store = ref.read(storeRepoProvider).store;

    return Container(
        // child: ToggleButtons(children: children, isSelected: isSelected),
        );
  }
}
