import 'package:dart_json_mapper/dart_json_mapper.dart';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'package:moon/utils/logger.dart';
import 'package:moon/providers/const_dropdown.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:moon/serialization/input_mapping.dart';
import 'package:tuple/tuple.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';

class Const extends HookConsumerWidget {
  Const({
    Key? key,
    required this.treeNode,
  }) : super(key: key);

  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    log.v("rebuilding Const ${treeNode.node.key}");
    final focusNode = useFocusNode();
    final dropDownFocusNode = useFocusNode();
    // final provider = ref.watch(changesController);
    final store = ref.read(storeRepoProvider).store;

    ref.watch(
        nodeController.select((value) => value.keys == treeNode.node.key));

    ValueNotifier<String> dropDownValue =
        treeNode.node.value.additionalData == ""
            ? useState("string")
            : useState(treeNode.node.value.additionalData);

    // print("dropDownValue $dropDownValue");

    //workaround so child widgets don't rebuild on dropdown change but when it when widget rebuilds
    final untrackedDropDownValue = treeNode.node.value.additionalData == ""
        ? "string"
        : treeNode.node.value.additionalData;

    // print("untrackedDropDownValue $untrackedDropDownValue");

    final valueList = ref.read(dropDownValues(treeNode));
    // print("valuelist $valueList");

    useEffect(() {
      dropDownValue.addListener(() {
        final MapEntry<String, Tuple3<String, int, int>> choice =
            valueList.entries.firstWhere((element) {
          // print("element.value.item1 ${element.value.item1}");
          // print("dropDownValue.value ${dropDownValue.value}");
          return element.value.item1 == dropDownValue.value;
        });
        // call setJson
        //update dimensions

        // print("choice $choice");
        // print("add listener dropDownValue $dropDownValue");
        store
            .msgUpdateDimensions(
          treeNode.node.key,
          dropDownValue.value,
          choice.value.item2, //width
          choice.value.item3, //height
        )
            .then((updated_dimensions) {
          // choice.value.item4.call();

          // unfocus dropdown
          dropDownFocusNode.unfocus();

          // set dimensions on repo

          // height.value = jsonDecode(value.data!)["height"];
          // width.value = jsonDecode(value.data!)["width"];

          // print(height);
          // print(width);
        });
      });

      return; // You need this return if you have missing_return lint
    }, [dropDownValue]);

    List<DropdownMenuItem<String>> dropDownList = valueList
        .map<String, DropdownMenuItem<String>>(
          (k, v) {
            return MapEntry(
              k,
              DropdownMenuItem(child: Text(k), value: v.item1),
            );
          },
        )
        .values
        .toList();
    // print(dropDownList);

    return Container(
      width: treeNode.node.value.width - 120,
      child: Padding(
        padding: const EdgeInsetsDirectional.fromSTEB(10, 10, 10, 10),
        child: Container(
          // decoration: BoxDecoration(
          //   color: Color(0xFFEEEEEE),
          //   border: Border.all(
          //     color: Color(0xFF258ED5),
          //   ),
          // // ),
          // width: treeNode.node.value.width - 120,
          // height: treeNode.node.value.height - 120,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                width: treeNode.node.value.width - 120,
                height: 50,
                child: DropdownButton(
                  focusNode: dropDownFocusNode,
                  isExpanded: true,
                  items: dropDownList,
                  onChanged: (value) {
                    // print("on changed value $value");
                    dropDownValue.value = value.toString();
                    // save to db and read from db
                    // store.msgSetAdditionalData(
                    //   treeNode.node.key,
                    //   value.toString(),
                    // );

                    // focusNode.requestFocus(); // FIXME
                  },
                  value: dropDownValue.value,
                ),
              ),
              // read current treeNode and provide accurate child widget
              buildChildField(untrackedDropDownValue, treeNode, focusNode)
                  .call(),
            ],
            // ),
          ),
        ),
      ),
    );
  }
}

// type should match the Value in Sunshine Solana
String createJson<T>(T value, String nodeId, [String? type]) {
  Map<String, T> inputProperties;
  String outer;
  Map<String, dynamic> outerMap;

  if (type != null) {
    inputProperties = {type: value};
    final outerMap = {"Const": inputProperties};
    outer = JsonMapper.serialize(InputProperties(outerMap));
    final combined = {"nodeId": nodeId, "text": outer};
    // print(combined);

    return JsonMapper.serialize(InputProperties(combined));
  } else {
    // final inputProperties = {
    //   "nodeId": treeNode.node.key,
    //   "text": text,
    // };
    // print(inputProperties);
    // String output = JsonMapper.serialize(InputProperties(inputProperties));

    // outerMap = value as Map<String, dynamic>;
    final outerMap = {"Const": value};
    outer = JsonMapper.serialize(InputProperties(outerMap));

    final combined = {"nodeId": nodeId, "text": outer};
    // print(combined);

    return JsonMapper.serialize(InputProperties(combined));
    // JsonMapper.serialize(InputProperties(value as Map<String, dynamic>));
  }
  // var input = JsonMapper.serialize(InputProperties(inputProperties));
}

final formKey = GlobalKey<FormBuilderState>();
