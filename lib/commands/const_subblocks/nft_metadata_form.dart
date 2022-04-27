import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:recase/recase.dart';
import 'package:moon/commands/const.dart';
import 'package:moon/providers/store_provider.dart';

import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:file_picker/file_picker.dart';

class NftMetadataForm extends HookConsumerWidget {
  NftMetadataForm({
    Key? key,
    required this.treeNode,
    required this.focusNode,
  }) : super(key: key);

  final TreeNode treeNode;
  final FocusNode focusNode;
  void _onChanged(dynamic val) => debugPrint(val.toString());

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final focusNode = useFocusNode();
    // final scrollController = useScrollController();
    // var controller = useTextEditingController(text: treeNode.node.value.text);
    // final store = ref.read(storeRepoProvider).store;
    // final filePath = useState("");
    final filePathController = useTextEditingController();

    return Container(
        child: Column(
      children: [
        FormBuilder(
          key: formKey,
          autovalidateMode: AutovalidateMode.disabled,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              FormBuilderTextField(
                name: 'Name',
                decoration: InputDecoration(
                  labelText: 'Name',
                ),
                minLines: 1,
                maxLines: 1,
                onChanged: _onChanged,
                // // valueTransformer: (text) => num.tryParse(text),
                // validator: FormBuilderValidators.compose([
                //   FormBuilderValidators.required(context),
                //   FormBuilderValidators.numeric(context),
                //   FormBuilderValidators.max(context, 70),
                // ]),
                keyboardType: TextInputType.text,
              ),
              FormBuilderTextField(
                name: 'Symbol',
                decoration: InputDecoration(
                  labelText: 'Symbol',
                ),
                minLines: 1,
                maxLines: 1,
                onChanged: _onChanged,
                // // valueTransformer: (text) => num.tryParse(text),
                // validator: FormBuilderValidators.compose([
                //   FormBuilderValidators.required(context),
                //   FormBuilderValidators.numeric(context),
                //   FormBuilderValidators.max(context, 70),
                // ]),
                keyboardType: TextInputType.text,
              ),
              FormBuilderTextField(
                name: 'name',
                decoration: InputDecoration(
                  labelText: 'Description',
                ),
                minLines: 2,
                maxLines: 3,
                onChanged: _onChanged,
                // // valueTransformer: (text) => num.tryParse(text),
                // validator: FormBuilderValidators.compose([
                //   FormBuilderValidators.required(context),
                //   FormBuilderValidators.numeric(context),
                //   FormBuilderValidators.max(context, 70),
                // ]),
                keyboardType: TextInputType.multiline,
              ),
              FormBuilderTextField(
                name: 'Seller Fee Basis',
                decoration: InputDecoration(
                  labelText: 'Seller Fee Basis, e.g. enter 500 for 5%',
                ),
                onChanged: _onChanged,
                // // valueTransformer: (text) => num.tryParse(text),
                // validator: FormBuilderValidators.compose([
                //   FormBuilderValidators.required(context),
                //   FormBuilderValidators.numeric(context),
                //   FormBuilderValidators.max(context, 70),
                // ]),
                keyboardType: TextInputType.number,
              ),
              const SizedBox(height: 20),
              Text("File URL", style: TextStyle(fontSize: 20)),
              const SizedBox(height: 10),
              TextButton.icon(
                icon: const Icon(Icons.folder_open_rounded),
                label: Text("Open file picker"),
                onPressed: () async {
                  FilePickerResult? result =
                      await FilePicker.platform.pickFiles(type: FileType.any);

                  if (result != null) {
                    PlatformFile file = PlatformFile(
                        path: result.files.single.path,
                        name: result.files.single.name,
                        size: result.files.single.size);
                    filePathController.value =
                        TextEditingValue(text: result.files.single.path!);
                    // print(file);
                  } else {
                    // User canceled the picker
                  }
                },
              ),
              FormBuilderTextField(
                name: 'File Path',
                decoration: InputDecoration(
                  labelText: 'File Path',
                ),
                minLines: 2,
                maxLines: 2,
                onChanged: _onChanged,
                controller: filePathController,
                // // valueTransformer: (text) => num.tryParse(text),
                // validator: FormBuilderValidators.compose([
                //   FormBuilderValidators.required(context),
                //   FormBuilderValidators.numeric(context),
                //   FormBuilderValidators.max(context, 70),
                // ]),
                keyboardType: TextInputType.multiline,
              ),
              FormBuilderTextField(
                name: 'Animation URL',
                decoration: InputDecoration(
                  labelText: 'Animation URL',
                ),
                minLines: 2,
                maxLines: 2,
                onChanged: _onChanged,
                // // valueTransformer: (text) => num.tryParse(text),
                // validator: FormBuilderValidators.compose([
                //   FormBuilderValidators.required(context),
                //   FormBuilderValidators.numeric(context),
                //   FormBuilderValidators.max(context, 70),
                // ]),
                keyboardType: TextInputType.multiline,
              ),
              FormBuilderTextField(
                name: 'External URL',
                decoration: InputDecoration(
                  labelText: 'External URL',
                ),
                minLines: 2,
                maxLines: 2,
                onChanged: _onChanged,
                // // valueTransformer: (text) => num.tryParse(text),
                // validator: FormBuilderValidators.compose([
                //   FormBuilderValidators.required(context),
                //   FormBuilderValidators.numeric(context),
                //   FormBuilderValidators.max(context, 70),
                // ]),
                keyboardType: TextInputType.multiline,
              ),
              Row(
                children: <Widget>[
                  Expanded(
                    child: MaterialButton(
                      color: Theme.of(context).colorScheme.secondary,
                      onPressed: () {
                        if (formKey.currentState?.saveAndValidate() ?? false) {
                          debugPrint(formKey.currentState?.value.toString());
                        } else {
                          debugPrint(formKey.currentState?.value.toString());
                          debugPrint('validation failed');
                        }
                      },
                      child: const Text(
                        'Submit',
                        style: TextStyle(color: Colors.white),
                      ),
                    ),
                  ),
                  const SizedBox(width: 20),
                  Expanded(
                    child: OutlinedButton(
                      onPressed: () {
                        formKey.currentState?.reset();
                      },
                      // color: Theme.of(context).colorScheme.secondary,
                      child: Text(
                        'Reset',
                        style: TextStyle(
                            color: Theme.of(context).colorScheme.secondary),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        )
      ],
    ));
  }
}

/*
  pub symbol: String,
    pub description: String,
    pub seller_fee_basis_points: u16,
    pub image: String,
    pub animation_url: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Vec<NftMetadataAttribute>,
    pub collection: Option<NftMetadataCollection>,
    pub properties: NftMetadataProperties,
 */
