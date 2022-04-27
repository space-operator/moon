import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';
import 'package:url_launcher/url_launcher.dart';

class Print extends HookConsumerWidget {
  Print({Key? key, required this.treeNode})
      : this.input = "",
        super(key: key);

  final String input;
  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    Future<void> _copyToClipboard(text) async {
      await Clipboard.setData(ClipboardData(text: text));
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: const Text('Copied to clipboard', textAlign: TextAlign.center),
      ));
      // Scaffold.of(context).showSnackBar(snackbar)
    }

    Future<void> _launchInBrowser(String url) async {
      if (!await launch(
        url,
        forceSafariVC: false,
        forceWebView: false,
        // headers: <String, String>{'my_header_key': 'my_header_value'},
      )) {
        throw 'Could not launch $url';
      }
    }

    final printOutput = treeNode.node.value.printOutput;
    String? textType;
    String? textContent;

    String? fullUrl;
    final solanaNet = ref.read(storeRepoProvider).store.view.solanaNet;
    String urlExtension = "";

    switch (solanaNet) {
      case rid.SolanaNet.Devnet:
        {
          urlExtension = "?cluster=devnet";
        }
        break;
      case rid.SolanaNet.Testnet:
        urlExtension = "?cluster=testnet";
        {}
        break;
      case rid.SolanaNet.Mainnet:
        {
          urlExtension = "";
        }
        break;
      default:
    }

    if (printOutput.contains("&&&")) {
      textType = printOutput.split("&&&")[0];
      textContent = printOutput.split("&&&")[1];

      // print(textType);
      switch (textType) {
        case "pubkey":
          {
            fullUrl = "https://explorer.solana.com/address/" +
                textContent +
                urlExtension;
          }
          break;
        case "success":
          {
            fullUrl =
                "https://explorer.solana.com/tx/" + textContent + urlExtension;
          }
          break;
        default:
      }
    }

    return Container(
      child: Center(
        child: treeNode.node.value.runState == rid.RunStateView.Success
            ? Row(
                children: [
                  Expanded(
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: SelectableText(
                        textContent ?? "",
                        textAlign: TextAlign.right,
                        style: const TextStyle(fontSize: 18),
                      ),
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.fromLTRB(5, 0, 15, 0),
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        if (fullUrl != null)
                          Container(
                            width: 30,
                            child: IconButton(
                                icon: const Icon(Icons.open_in_new),
                                onPressed: () {
                                  _launchInBrowser(fullUrl!);
                                }),
                          ),
                        Divider(),
                        Container(
                          width: 30,
                          child: IconButton(
                              icon: const Icon(Icons.copy),
                              onPressed: () {
                                _copyToClipboard(textContent);
                              }),
                        ),
                      ],
                    ),
                  )
                ],
              )
            : treeNode.node.value.runState == rid.RunStateView.Failed
                ? ListTile(
                    trailing: IconButton(
                        icon: const Icon(Icons.copy),
                        onPressed: () {
                          _copyToClipboard(treeNode.node.value.error);
                        }),
                    title: SelectableText(
                      treeNode.node.value.error,
                      textAlign: TextAlign.right,
                      style: const TextStyle(fontSize: 18),
                    ),
                  )
                : null,
      ),
    );
  }
}

/*
Card(
        color: node.value.success == "success"
            ? Color(Colors.green.value)
            : Color(0xFFF5F5F5),
        child: Container(
          width: node.value.width.toDouble(),
          height: node.value.height.toDouble(),
          decoration: BoxDecoration(
            color: node.value.success == "success"
                ? Color(Colors.green.value)
                : Color(0xFFEEEEEE),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.max,
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Expanded(
                child: Row(
                  mainAxisSize: MainAxisSize.max,
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    Column(
                      mainAxisSize: MainAxisSize.max,
                      children: inputs,
                    ),
                    Expanded(
                      child: Padding(
                        padding: EdgeInsetsDirectional.fromSTEB(10, 5, 5, 10),
                        child: Column(
                          mainAxisSize: MainAxisSize.max,
                          mainAxisAlignment: MainAxisAlignment.start,
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            Container(
                              width: 400,
                              height: 75,
                              decoration: BoxDecoration(
                                color: Color(Colors.white.value),
                                border: Border.all(
                                  color: Color(0xFF258ED5),
                                ),
                              ),
                              child: Padding(
                                padding: EdgeInsetsDirectional.fromSTEB(
                                    10, 5, 5, 10),
                                child: Column(
                                  mainAxisSize: MainAxisSize.max,
                                  mainAxisAlignment: MainAxisAlignment.start,
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Padding(
                                      padding: EdgeInsetsDirectional.fromSTEB(
                                          5, 0, 5, 0),
                                      child: Text(
                                        '$input',
                                        // style: FlutterFlowTheme
                                        //     .bodyText1
                                        //     .override(
                                        //   fontFamily: 'Poppins',
                                        //   fontWeight: FontWeight.w600,
                                        // ),
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),

 */
