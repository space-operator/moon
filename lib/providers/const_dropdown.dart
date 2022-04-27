import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:moon/commands/const_subblocks/bool_field.dart';
import 'package:moon/commands/const_subblocks/file_picker.dart';
import 'package:moon/commands/const_subblocks/custom_type_text_field.dart';
import 'package:moon/commands/const_subblocks/json_text_field.dart';
// import 'package:moon/commands/const_subblocks/nft_metadata_form.dart';
import 'package:moon/commands/const_subblocks/numbers_field.dart';
import 'package:moon/commands/const_subblocks/operator.dart';
import 'package:moon/commands/const_subblocks/pubkey.dart';
import 'package:moon/commands/const_subblocks/seed_phrase_field.dart';
import 'package:moon/commands/const_subblocks/string_field.dart';
import 'package:moon/providers/store_provider.dart';

import 'package:tuple/tuple.dart';

final dropDownValues =
    Provider.family<Map<String, Tuple3<String, int, int>>, TreeNode>(
        (ref, treeNode) {
  return const {
    "String": Tuple3("string", 300, 300),
    "Pubkey": Tuple3("pubkey", 300, 200),
    "JSON": Tuple3("json", 400, 500),

    "File Picker": Tuple3("file_picker", 300, 300),
    "Boolean, True": Tuple3("bool_true", 300, 110),
    "Boolean, False": Tuple3("bool_false", 300, 110),
    "Operator": Tuple3("operator", 300, 170),
    // "NFT Metadata": Tuple3("nft", 300, 600),
    "Seed Phrase": Tuple3("seed", 400, 220),
    "Number, i64": Tuple3("i64", 300, 175),
    "Number, u8": Tuple3("u8", 300, 175),
    "Number, u16": Tuple3("u16", 300, 175),
    "Number, u64": Tuple3("u64", 300, 175),
    "Number, f32": Tuple3("f32", 300, 175),
    "Number, f64": Tuple3("f64", 300, 175),
    "Custom Type": Tuple3("custom", 400, 500),
  };
});

/// OPERATORS
final operatorValues = Provider<Map<String, Tuple2<String, String>>>((ref) {
  return const {
    "Equal": Tuple2("Eq", "="),
    "Not equal": Tuple2("NotEq", "!="),
    "Greater": Tuple2("Greater", ">"),
    "Greater or eq": Tuple2("GreaterEq", ">="),
    "Less than": Tuple2("Less", "<"),
    "Less or eq": Tuple2("LessEq", "<="),
  };
});

/// Child Field Router
Function buildChildField(
    String? fieldType, TreeNode treeNode, FocusNode focusNode) {
  if (fieldType == null) {
    return (List<dynamic> inputs) => Container();
  } else {
    final widgetStore = <String, Function>{
      "custom": () => CustomTypeTextField(treeNode: treeNode),
      "json": () => JsonTextField(treeNode: treeNode),
      "bool_true": () =>
          BoolField(treeNode: treeNode, focusNode: focusNode, boolValue: true),
      "bool_false": () =>
          BoolField(treeNode: treeNode, focusNode: focusNode, boolValue: false),
      "string": () => StringTextField(treeNode: treeNode, focusNode: focusNode),
      "pubkey": () => PubkeyField(treeNode: treeNode, focusNode: focusNode),
      "file_picker": () =>
          FilePickerField(treeNode: treeNode, focusNode: focusNode),
      "operator": () => OperatorField(treeNode: treeNode, focusNode: focusNode),
      // "nft": () => NftMetadataForm(treeNode: treeNode, focusNode: focusNode),
      "seed": () => SeedTextField(treeNode: treeNode),
      "i64": () =>
          NumberTextField(treeNode: treeNode, numberType: "I64", numberIs: int),
      "u8": () =>
          NumberTextField(treeNode: treeNode, numberType: "U8", numberIs: int),
      "u16": () =>
          NumberTextField(treeNode: treeNode, numberType: "U16", numberIs: int),
      "u64": () =>
          NumberTextField(treeNode: treeNode, numberType: "U64", numberIs: int),
      "f32": () => NumberTextField(
          treeNode: treeNode, numberType: "F32", numberIs: double),
      "f64": () => NumberTextField(
          treeNode: treeNode, numberType: "F64", numberIs: double),
    };

    return widgetStore.entries
        .firstWhere((element) => element.key == fieldType)
        .value;
  }
}
