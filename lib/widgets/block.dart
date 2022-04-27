import 'package:flutter/material.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/utils/logger.dart';
import 'package:moon/providers/store_provider.dart';

abstract class SuperBlock extends HookConsumerWidget {
  final List<Widget>? children;

  final MapEntry<String, rid.NodeView>? node;

  final bool? selected;

  SuperBlock({Key? key, this.node, this.children, this.selected});
}

class Block extends SuperBlock {
  Block({
    Key? key,
    required this.treeNode,
  }) : super(key: key);

  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print(
    //     "rebuilding Block ${treeNode.node.value.widgetType} ${treeNode.node.key}");
    // ref.watch(treeNodeController);
    final provider = ref.watch(
      treeNodeController.select((map) {
        if (map.values.isNotEmpty)
          return map.values.first
              .where(
                (list) => list.node.key == treeNode.node.key,
              )
              .first
              .node
              .value;
      }),
    );
    // ref.watch(nodeController);

    // print("rebuilding block $key");

    // print(treeNode);
    // get the tree node based on the id
    // final newTreeNode = ref.read(treeNodeRepoProvider).get(treeNode.node.key);

    final newTreeNode = treeNode;
    // final provider = ref.watch(
    //   nodeController.select(
    //     (hashmap) => hashmap.entries
    //         .where((element) => element.key == newTreeNode.node.key)
    //         .toList()[0]
    //         .value,
    //   ),
    // );
    // final provider2 = ref.watch(treeNodeController.select(
    //   (treeRepo) {
    //     return treeRepo.treeNodes
    //         .where((treeNode) => treeNode.node.key == newTreeNode.node.key)
    //         .toList();
    //   },
    // ));
    // print(provider2);
    // final nodeFrom = provider.entries.firstWhere((element) {
    //   return element.key == this.node.key;
    // });
    // print(selected);
    return
        // ProviderScope(
        //   overrides: [
        //     treeNodeProvider.setupOverride();
        //   ], //not working
        //   child:
        Positioned(
      height: newTreeNode.node.value.height.toDouble() +
          70, //FIXME workaround to show a button  on error
      width: newTreeNode.node.value.width.toDouble(),
      left: newTreeNode.node.value.x.toDouble(),
      top: newTreeNode.node.value.y.toDouble(),
      child: Container(
        color: Colors.transparent,

        // margin: EdgeInsets.all(5),
        // color: Colors.transparent, // ? Colors.amber : Colors.white,
        child: Stack(
          children: [
            ...newTreeNode.children!,
          ],
        ),
      ),
      // ),
    );
  }
}
