import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:tuple/tuple.dart';

final help = Provider<Map<int, Tuple3<String, String, String>>>(
  (ref) {
    return const {
      // Name, Description, Relevant nodes used, Filename
      1: Tuple3(
        "Basic UI",
        "Using mouse, double-click to create a command, left-click and drag to move nove, right-click to pan, scroll to zoom.\n\nFor Mac trackpads, you can switch the input processing in the main menu. The trackpad will be for panning and use the UI zoom buttons on the top bar",
        "ui",
      ),
      2: Tuple3(
        "Templates",
        "The template section has many examples to illustrate use cases and how specific commands work.\n\nCheck the bookmarks as they will provide additional guidance for each template.\nSome template are ready to run, others need to you input some data.",
        "templates",
      ),
      3: Tuple3(
        "Add/Remove Commands",
        "Double-click on canvas and type command name to create a command.\nClick on ellipsis menu to show delete button.",
        "commands",
      ),
      4: Tuple3(
        "Add/Remove Edges",
        "Drag from an input to an output to create an edge.\nTo remove an edge, repeat the creation process, drag an input to an output to remove the edge, i.e. how one would unplug a power extension",
        "edges",
      ),
      5: Tuple3(
        "Add/Remove Bookmarks",
        "Select one or multiple command, and add a name to bookmark them.\nYou can find the saved bookmarks in the left expandable panel.\nSwipe left to remove a bookmark",
        "bookmarks",
      ),
      6: Tuple3(
        "Command Help Section",
        "Basic description for each command. Filter for a specific command by typing it's name or by selecting an existing command.",
        "command_help",
      ),
      7: Tuple3(
        "Running a Flow",
        "Press the deploy/undeploy to start/stop a flow.\nIterate on Devnet, Testnet before you switch to Mainnet.\nMake sure to update the commands for the endpoints before running.\nSome commands are only available on Devnet, Testnet, like Airdrop",
        "flow",
      ),
    };
  },
);
