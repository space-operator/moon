import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:moon/nodes/command_widget.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:moon/widgets/text_input.dart';

import 'package:moon/commands/const.dart';

import 'package:moon/nodes/command_widget.dart';
import 'package:moon/commands/print.dart';
import 'package:tuple/tuple.dart';

import './widgets/block.dart';

import 'utils/logger.dart';

SuperBlock WidgetChooser(TreeNode treeNode,
    [nodeType, inputNodes, outputNodes, parentId, ref
    // storedContext,
    ]) {
  HookConsumerWidget? _widget;

  switch (nodeType) {
    case "WidgetBlock":
      {
        _widget = Block(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
        );
      }
      break;

    case "WidgetTextInput":
      {
        log.v("adding text input");

        _widget = TextInput(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          parentId: parentId,
          // context: storedContext,
        );
      }
      break;
    case "DummyEdgeHandle":
    case "WidgetInput":
    case "WidgetOutput":
      {}
      break;
    //
    case "Const":
      {
        _widget = CommandWidget(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          child: Const(
            treeNode: treeNode,
            key: ObjectKey(treeNode.node.value),
          ),
          parentId: parentId,
        );
      }

      break;
    case "Print":
      {
        _widget = CommandWidget(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          child: Print(
            treeNode: treeNode,
            key: ObjectKey(treeNode.node.value),
          ),
          parentId: parentId,
        );
      }
      break;
    case "JsonExtract":
    case "JsonInsert":
    case "HttpRequest":
    case "IpfsUpload":
    case "IpfsNftUpload":
    case "Wait":
    case "Branch":
    //
    case "CreateMintAccount":
    case "CreateTokenAccount":
    case "GenerateKeypair":
    case "MintToken":
    case "TransferToken":
    case "TransferSolana":
    case "RequestAirdrop":
    case "GetBalance":
    //
    case "CreateMetadataAccounts":
    case "CreateMasterEdition":
    case "UpdateMetadataAccounts":
    case "VerifyCollection":
    case "ApproveCollectionAuthority":
    case "SignMetadata":
    case "Utilize":
    case "ApproveUseAuthority":
    case "GetLeftUses":
    // case "ArweaveNftUpload":
    // case "ArweaveUpload":
    case "ArweaveFileUpload":
    case "ArweaveNftUpload":
      {
        _widget = CommandWidget(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          parentId: parentId,
        );
      }
      break;
    default:
      {
        // return Text("No Matching cases: ${data.properties}");
      }
  }

  return _widget as SuperBlock;
}
// }
// CommandWidget(
//               key: UniqueKey(),
//               treeNode: treeNode,
//               inputs: inputNodes,
//               outputs: outputNodes,
//               label: nodeType,
//               parentId: parentId,
//             );

// final commandProvider = Provider.family<CommandWidget, Tuple5>(
//   (ref, tuple) {
//     return CommandWidget(
//       key: UniqueKey(),
//       treeNode: tuple.item1,
//       inputs: tuple.item2,
//       outputs: tuple.item3,
//       label: tuple.item4,
//       parentId: tuple.item5,
//     );
//   },
// );
