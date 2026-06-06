import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

void main() {
  test(
    'presentation entrypoints live under features instead of screens shims',
    () {
      expect(Directory('lib/src/screens').existsSync(), isFalse);

      const staleImportNeedle =
          'src/'
          'screens/';
      final staleImports = Directory('test')
          .listSync(recursive: true)
          .whereType<File>()
          .where((file) => file.path.endsWith('_test.dart'))
          .where((file) => file.readAsStringSync().contains(staleImportNeedle))
          .map((file) => file.path)
          .toList();

      expect(staleImports, isEmpty);
    },
  );

  test('app shell does not prop drill workspace state into wallet home', () {
    final shell = File('lib/src/app/wallet_app_shell.dart').readAsStringSync();
    final home = File('lib/src/app/wallet_home.dart').readAsStringSync();

    expect(shell, isNot(contains('WalletApi')));
    expect(shell, isNot(contains('walletApiProvider')));
    expect(shell, isNot(contains('wallets: workspace.wallets')));
    expect(shell, isNot(contains('assets: workspace.assets')));
    expect(shell, isNot(contains('activity: workspace.activity')));

    expect(home, contains('class WalletHome extends ConsumerStatefulWidget'));
    expect(home, contains('const WalletHome({super.key});'));
    expect(home, contains('walletWorkspaceControllerProvider'));
    expect(home, isNot(contains('final WalletApi api;')));
    expect(home, isNot(contains('final List<WalletSummary> wallets;')));
  });

  test('wallet home separates adaptive shell from page wiring', () {
    final app = Directory('lib/src/app');
    final home = File('${app.path}/wallet_home.dart').readAsStringSync();
    final pages = File('${app.path}/wallet_home_pages.dart');
    final mobileNavigation = File('${app.path}/wallet_mobile_navigation.dart');

    expect(pages.existsSync(), isTrue);
    expect(mobileNavigation.existsSync(), isTrue);
    expect(home, contains("import 'wallet_home_pages.dart';"));
    expect(home, contains("import 'wallet_mobile_navigation.dart';"));
    expect(home, isNot(contains('WalletsScreen(')));
    expect(home, isNot(contains('AssetsScreen(')));
    expect(home, isNot(contains('TransferScreen(')));
    expect(home, isNot(contains('NavigationDestination(')));
  });

  test('activity presentation splits list shell from row formatting', () {
    final presentation = Directory('lib/src/features/activity/presentation');
    final list = File(
      '${presentation.path}/activity_list.dart',
    ).readAsStringSync();

    expect(File('${presentation.path}/activity_row.dart').existsSync(), isTrue);
    expect(
      File('${presentation.path}/activity_details.dart').existsSync(),
      isTrue,
    );
    expect(list, isNot(contains('class ActivityRow')));
    expect(list, isNot(contains('_parseTransferSummary')));
  });

  test('activity row delegates reusable row parts', () {
    final presentation = Directory('lib/src/features/activity/presentation');
    final row = File('${presentation.path}/activity_row.dart');
    final source = row.readAsStringSync();
    final textColumn = File('${presentation.path}/activity_text_column.dart');
    final textColumnSource = textColumn.existsSync()
        ? textColumn.readAsStringSync()
        : '';

    for (final name in <String>[
      'activity_amount_label',
      'activity_icon',
      'activity_text_column',
    ]) {
      expect(File('${presentation.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("import '$name.dart';"));
    }
    expect(
      File('${presentation.path}/activity_status_chip.dart').existsSync(),
      isTrue,
    );
    expect(textColumnSource, contains("import 'activity_status_chip.dart';"));

    expect(source, contains('class ActivityRow'));
    expect(source, contains('ActivityTextColumn('));
    expect(source, contains('ActivityAmountLabel('));
    expect(source, contains('ActivityIcon('));
    expect(source, isNot(contains('class _ActivityTextColumn')));
    expect(source, isNot(contains('class _AmountLabel')));
    expect(source, isNot(contains('class _ActivityIcon')));
    expect(source, isNot(contains('class _StatusChip')));
  });

  test('activity details delegates transfer parsing and labels', () {
    final presentation = Directory('lib/src/features/activity/presentation');
    final details = File('${presentation.path}/activity_details.dart');
    final parser = File('${presentation.path}/activity_summary_parser.dart');
    final labels = File('${presentation.path}/activity_labels.dart');
    final detailsSource = details.readAsStringSync();
    final parserSource = parser.existsSync() ? parser.readAsStringSync() : '';
    final labelsSource = labels.existsSync() ? labels.readAsStringSync() : '';

    expect(parser.existsSync(), isTrue);
    expect(labels.existsSync(), isTrue);
    expect(detailsSource, contains("import 'activity_summary_parser.dart';"));
    expect(detailsSource, contains("import 'activity_labels.dart';"));
    expect(detailsSource, isNot(contains('RegExp(')));
    expect(detailsSource, isNot(contains('String shortActivityHash')));
    expect(detailsSource, isNot(contains('String activityTitleCase')));
    expect(detailsSource, isNot(contains('_kindLabel')));
    expect(parserSource, contains('parseActivityTransferSummary'));
    expect(labelsSource, contains('shortActivityHash'));
    expect(labelsSource, contains('activityTitleCase'));
  });

  test('settings screen delegates route, body, and security actions', () {
    final settings = Directory('lib/src/features/settings');
    final barrel = File('${settings.path}/settings_screen.dart');
    final route = File('${settings.path}/settings_screen_route.dart');
    final body = File('${settings.path}/settings_screen_body.dart');
    final actions = File('${settings.path}/settings_security_actions.dart');
    final barrelSource = barrel.readAsStringSync();
    final routeSource = route.existsSync() ? route.readAsStringSync() : '';

    for (final file in <File>[route, body, actions]) {
      expect(file.existsSync(), isTrue);
    }

    expect(barrelSource, contains("export 'settings_screen_route.dart';"));
    expect(barrelSource, isNot(contains('class SettingsScreen')));
    expect(routeSource, contains('ConsumerStatefulWidget'));
    expect(routeSource, contains('walletApiProvider'));
    expect(routeSource, contains('SettingsScreenBody('));
    expect(routeSource, contains('SettingsSecurityActions('));
    expect(routeSource, isNot(contains('Scaffold(')));
    expect(routeSource, isNot(contains('promptMasterPassword(')));
    expect(routeSource, isNot(contains('showAdvancedSecurityDialog(')));
    expect(routeSource, isNot(contains('showAlternateUnlockDialog(')));
    expect(routeSource, isNot(contains('ScaffoldMessenger.of')));
    expect(routeSource, isNot(contains('final WalletApi api;')));
    expect(routeSource, isNot(contains('final VoidCallback onLock;')));
    expect(routeSource, isNot(contains('final List<WalletSummary> wallets;')));
  });

  test('settings sections are split by settings category', () {
    final settings = Directory('lib/src/features/settings');
    final barrel = File('${settings.path}/settings_sections.dart');
    final source = barrel.readAsStringSync();

    for (final name in <String>[
      'settings_network_section',
      'settings_privacy_section',
      'settings_security_section',
    ]) {
      expect(File('${settings.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("export '$name.dart';"));
    }

    expect(source, isNot(contains('class SettingsNetworkSection')));
    expect(source, isNot(contains('class SettingsPrivacySection')));
    expect(source, isNot(contains('class SettingsSecuritySection')));
    expect(source, isNot(contains('FutureBuilder')));
    expect(source, isNot(contains('SwitchListTile')));
  });

  test('network privacy dialog separates route, panel, and actions', () {
    final settings = Directory('lib/src/features/settings');
    final barrel = File('${settings.path}/network_privacy_dialog.dart');
    final route = File('${settings.path}/network_privacy_dialog_route.dart');
    final panel = File('${settings.path}/network_privacy_dialog_panel.dart');
    final actions = File(
      '${settings.path}/network_privacy_dialog_actions.dart',
    );
    final barrelSource = barrel.readAsStringSync();
    final routeSource = route.existsSync() ? route.readAsStringSync() : '';
    final panelSource = panel.existsSync() ? panel.readAsStringSync() : '';
    final actionsSource = actions.existsSync()
        ? actions.readAsStringSync()
        : '';

    for (final file in <File>[route, panel, actions]) {
      expect(file.existsSync(), isTrue);
    }

    expect(
      barrelSource,
      contains("export 'network_privacy_dialog_route.dart';"),
    );
    expect(barrelSource, isNot(contains('StatefulWidget')));
    expect(barrelSource, isNot(contains('AlertDialog(')));
    expect(routeSource, contains('showNetworkPrivacyDialog'));
    expect(routeSource, contains('NetworkPrivacyDialogPanel('));
    expect(routeSource, isNot(contains('StatefulWidget')));
    expect(routeSource, isNot(contains('NetworkPrivacyDraftController')));
    expect(panelSource, contains('class NetworkPrivacyDialogPanel'));
    expect(panelSource, contains('NetworkPrivacyDialogActions('));
    expect(panelSource, isNot(contains('OutlinedButton.icon')));
    expect(panelSource, isNot(contains('FilledButton.icon')));
    expect(actionsSource, contains('class NetworkPrivacyDialogActions'));
    expect(actionsSource, contains('OutlinedButton.icon'));
    expect(actionsSource, contains('FilledButton.icon'));
  });

  test('unlock screen delegates route, body, form, and biometrics', () {
    final session = Directory('lib/src/features/session/presentation');
    final barrel = File('${session.path}/unlock_screen.dart');
    final route = File('${session.path}/unlock_screen_route.dart');
    final body = File('${session.path}/unlock_screen_body.dart');
    final barrelSource = barrel.readAsStringSync();
    final routeSource = route.existsSync() ? route.readAsStringSync() : '';
    final bodySource = body.existsSync() ? body.readAsStringSync() : '';

    for (final name in <String>[
      'unlock_biometric_button',
      'unlock_brand_header',
      'unlock_password_form',
      'unlock_screen_body',
      'unlock_screen_route',
    ]) {
      expect(File('${session.path}/$name.dart').existsSync(), isTrue);
    }

    expect(barrelSource, contains("export 'unlock_screen_route.dart';"));
    expect(barrelSource, isNot(contains('class UnlockScreen')));
    expect(routeSource, contains('class UnlockScreen'));
    expect(routeSource, contains('UnlockScreenBody('));
    expect(routeSource, isNot(contains('Scaffold(')));
    expect(routeSource, isNot(contains('Image.asset(')));
    expect(routeSource, isNot(contains('TextField(')));
    expect(routeSource, isNot(contains('OutlinedButton.icon')));
    expect(bodySource, contains("import 'unlock_biometric_button.dart';"));
    expect(bodySource, contains("import 'unlock_brand_header.dart';"));
    expect(bodySource, contains("import 'unlock_password_form.dart';"));
  });

  test(
    'desktop sidebar splits shell from navigation and wallet list items',
    () {
      final app = Directory('lib/src/app');
      final sidebar = File(
        '${app.path}/wallet_sidebar.dart',
      ).readAsStringSync();

      expect(
        File('${app.path}/wallet_sidebar_navigation.dart').existsSync(),
        isTrue,
      );
      expect(
        File('${app.path}/wallet_sidebar_wallets.dart').existsSync(),
        isTrue,
      );
      expect(sidebar, isNot(contains('class _SidebarNavItem')));
      expect(sidebar, isNot(contains('class _SidebarWalletItem')));
    },
  );

  test('transfer form splits controls from preview presentation', () {
    final presentation = Directory('lib/src/features/transfers/presentation');
    final form = File(
      '${presentation.path}/transfer_form.dart',
    ).readAsStringSync();

    expect(
      File('${presentation.path}/transfer_asset_selector.dart').existsSync(),
      isTrue,
    );
    expect(
      File('${presentation.path}/transfer_action_bar.dart').existsSync(),
      isTrue,
    );
    expect(
      File('${presentation.path}/transfer_preview_card.dart').existsSync(),
      isTrue,
    );
    expect(form, isNot(contains('transfer.preview!')));
    expect(form, isNot(contains('_assetLabel')));
  });

  test('asset dialogs are split by token workflow', () {
    final assets = Directory('lib/src/features/assets/presentation');
    final barrel = File('${assets.path}/assets_dialogs.dart');
    final source = barrel.readAsStringSync();

    for (final name in <String>[
      'add_token_dialog',
      'asset_dialog_callbacks',
      'remove_token_dialog',
    ]) {
      expect(File('${assets.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("export '$name.dart';"));
    }

    expect(source, isNot(contains('AlertDialog')));
    expect(source, isNot(contains('StatefulBuilder')));
    expect(source, isNot(contains('TextField')));
    expect(source, isNot(contains('WalletApiException')));
  });

  test('assets screen delegates toolbar actions and body', () {
    final assets = Directory('lib/src/features/assets/presentation');
    final screen = File('${assets.path}/assets_screen.dart');
    final actions = File('${assets.path}/assets_screen_actions.dart');
    final body = File('${assets.path}/assets_screen_body.dart');
    final source = screen.readAsStringSync();
    final actionsSource = actions.existsSync()
        ? actions.readAsStringSync()
        : '';
    final bodySource = body.existsSync() ? body.readAsStringSync() : '';

    expect(actions.existsSync(), isTrue);
    expect(body.existsSync(), isTrue);
    expect(source, contains("import 'assets_screen_actions.dart';"));
    expect(source, contains("import 'assets_screen_body.dart';"));
    expect(source, contains('AssetsScreenActions('));
    expect(source, contains('AssetsScreenBody('));
    expect(source, isNot(contains('IconButton(')));
    expect(source, isNot(contains('AssetsList(')));
    expect(source, isNot(contains('showAddTokenDialog(')));
    expect(actionsSource, contains('class AssetsScreenActions'));
    expect(actionsSource, contains('IconButton('));
    expect(actionsSource, contains('showAddTokenDialog('));
    expect(bodySource, contains('class AssetsScreenBody'));
    expect(bodySource, contains('AssetsList('));
  });

  test('native wallet API implementation lives under core native', () {
    final rootApi = File('lib/src/native_wallet_api.dart').readAsStringSync();
    final coreApi = File('lib/src/core/native/native_wallet_api.dart');

    expect(coreApi.existsSync(), isTrue);
    expect(rootApi, contains("export 'core/native/native_wallet_api.dart';"));
    expect(rootApi, isNot(contains('class NativeWalletApi')));
    expect(rootApi, isNot(contains('core/native/clients/')));
  });

  test('native wallet API facade delegates domain methods to mixins', () {
    final native = Directory('lib/src/core/native');
    final facade = File('${native.path}/native_wallet_api.dart');
    final source = facade.readAsStringSync();
    final api = Directory('${native.path}/api');

    for (final name in <String>[
      'activity',
      'app',
      'assets',
      'auth',
      'multisig',
      'network',
      'transfers',
      'wallets',
    ]) {
      expect(File('${api.path}/native_${name}_api.dart').existsSync(), isTrue);
      expect(source, contains("import 'api/native_${name}_api.dart';"));
    }

    expect(source, contains('with'));
    expect(source, contains('NativeAuthApi'));
    expect(source, contains('NativeWalletsApi'));
    expect(source, isNot(contains('Future<WalletSummary> createWallet')));
    expect(source, isNot(contains('Future<AssetSummary> addCustomToken')));
    expect(source, isNot(contains('Future<TransferPreview> previewTransfer')));
    expect(source, isNot(contains('Future<NetworkPrivacySettings>')));
  });

  test('demo wallet API facade delegates domain methods to mixins', () {
    final coreApi = Directory('lib/src/core/api');
    final facade = File('${coreApi.path}/demo_wallet_api.dart');
    final source = facade.readAsStringSync();
    final demoApi = Directory('${coreApi.path}/demo/api');

    for (final name in <String>[
      'activity',
      'app',
      'assets',
      'auth',
      'multisig',
      'network',
      'transfers',
      'wallets',
    ]) {
      expect(
        File('${demoApi.path}/demo_${name}_api.dart').existsSync(),
        isTrue,
      );
      expect(source, contains("import 'demo/api/demo_${name}_api.dart';"));
    }

    expect(source, contains('with'));
    expect(source, contains('DemoAuthApi'));
    expect(source, contains('DemoWalletsApi'));
    expect(source, isNot(contains('Future<WalletSummary> createWallet')));
    expect(source, isNot(contains('Future<AssetSummary> addCustomToken')));
    expect(source, isNot(contains('Future<TransferPreview> previewTransfer')));
    expect(source, isNot(contains('Future<NetworkPrivacySettings>')));
  });

  test('wallet client wrappers are split by API domain', () {
    final coreApi = Directory('lib/src/core/api');
    final facade = File('${coreApi.path}/wallet_clients.dart');
    final source = facade.readAsStringSync();
    final clients = Directory('${coreApi.path}/clients');

    for (final name in <String>[
      'activity',
      'assets',
      'auth',
      'multisig',
      'network',
      'transfers',
      'wallets',
    ]) {
      expect(File('${clients.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("import 'clients/$name.dart';"));
    }

    expect(source, contains('class WalletClients'));
    expect(source, isNot(contains('class AuthClient')));
    expect(source, isNot(contains('class WalletsClient')));
    expect(source, isNot(contains('class NetworkClient')));
  });

  test('native wallets client delegates command groups to mixins', () {
    final clients = Directory('lib/src/core/native/clients');
    final client = File('${clients.path}/native_wallets_client.dart');
    final source = client.readAsStringSync();

    for (final name in <String>[
      'native_wallets_generation',
      'native_wallets_imports',
      'native_wallets_exports',
      'native_wallets_queries',
    ]) {
      final file = File('${clients.path}/$name.dart');
      expect(file.existsSync(), isTrue);
      expect(source, contains("part '$name.dart';"));
    }

    expect(source, contains('NativeWalletsGeneration'));
    expect(source, contains('NativeWalletsImports'));
    expect(source, contains('NativeWalletsExports'));
    expect(source, contains('NativeWalletsQueries'));
    expect(source, isNot(contains('Future<String> generateMnemonic')));
    expect(source, isNot(contains('Future<WalletSummary> create')));
    expect(source, isNot(contains('Future<KeystoreExport> exportKeystore')));
    expect(source, isNot(contains('Future<List<WalletSummary>> list')));
    expect(
      source,
      isNot(contains('Future<List<AccountSummary>> listAccounts')),
    );
  });

  test('native response parsing is split by API domain', () {
    final native = Directory('lib/src/core/native');
    final parserBarrel = File(
      '${native.path}/native_response_parsers.dart',
    ).readAsStringSync();
    final parsers = Directory('${native.path}/parsers');

    for (final name in <String>[
      'activity',
      'assets',
      'fields',
      'multisig',
      'network',
      'transfers',
      'wallets',
    ]) {
      expect(File('${parsers.path}/$name.dart').existsSync(), isTrue);
      expect(parserBarrel, contains("export 'parsers/$name.dart';"));
    }

    expect(parserBarrel, isNot(contains('WalletSummary(')));
    expect(parserBarrel, isNot(contains('NetworkSettings(')));
    expect(parserBarrel, isNot(contains('MultisigProposalSummary(')));
  });

  test('macos update installer handles protected app bundles', () {
    final source = File(
      'macos/Runner/MainFlutterWindow.swift',
    ).readAsStringSync();

    expect(source, contains('install_with_admin'));
    expect(source, contains('with administrator privileges'));
    expect(source, contains('quoted form of appPath'));
    expect(source, contains('Move Safe Wallet to /Applications'));
  });

  test('screen motion utilities are split by animation responsibility', () {
    final ui = Directory('lib/src/core/ui');
    final barrel = File('${ui.path}/screen_motion.dart').readAsStringSync();
    final motion = Directory('${ui.path}/screen_motion');

    for (final name in <String>[
      'animated_page_stack',
      'constants',
      'motion_entrance',
      'motion_preferences',
      'motion_switcher',
      'staggered_list_item',
    ]) {
      expect(File('${motion.path}/$name.dart').existsSync(), isTrue);
      expect(barrel, contains("export 'screen_motion/$name.dart';"));
    }

    expect(barrel, isNot(contains('class MotionEntrance')));
    expect(barrel, isNot(contains('class StaggeredListItem')));
    expect(barrel, isNot(contains('class AnimatedPageStack')));
    expect(barrel, isNot(contains('Timer?')));
  });

  test('network settings dialog delegates form and validation details', () {
    final settings = Directory('lib/src/features/settings');
    final dialog = File(
      '${settings.path}/network_settings_dialog.dart',
    ).readAsStringSync();

    expect(
      File('${settings.path}/network_settings_form.dart').existsSync(),
      isTrue,
    );
    expect(
      File('${settings.path}/network_settings_presets.dart').existsSync(),
      isTrue,
    );
    expect(
      File('${settings.path}/network_settings_validation.dart').existsSync(),
      isTrue,
    );
    expect(dialog, isNot(contains('class _UrlInput')));
    expect(dialog, isNot(contains('_networkValidationError')));
    expect(dialog, isNot(contains('_rpcUrlPresets')));
  });

  test('network settings form delegates controllers and URL inputs', () {
    final settings = Directory('lib/src/features/settings');
    final barrel = File('${settings.path}/network_settings_form.dart');
    final source = barrel.readAsStringSync();

    for (final name in <String>[
      'network_settings_form_body',
      'network_settings_form_controllers',
      'network_settings_url_input',
    ]) {
      expect(File('${settings.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("export '$name.dart';"));
    }

    expect(source, isNot(contains('class NetworkSettingsFormControllers')));
    expect(source, isNot(contains('class NetworkSettingsForm')));
    expect(source, isNot(contains('class _UrlInput')));
    expect(source, isNot(contains('TextField(')));
    expect(source, isNot(contains('NetworkSettingsDraft(')));
  });

  test('network privacy dialog delegates form controls and draft state', () {
    final settings = Directory('lib/src/features/settings');
    final dialog = File(
      '${settings.path}/network_privacy_dialog.dart',
    ).readAsStringSync();

    expect(
      File('${settings.path}/network_privacy_form.dart').existsSync(),
      isTrue,
    );
    expect(
      File('${settings.path}/network_privacy_state.dart').existsSync(),
      isTrue,
    );
    expect(dialog, isNot(contains('SwitchListTile')));
    expect(dialog, isNot(contains('SegmentedButton<String>')));
    expect(dialog, isNot(contains('_defaultTorProxyUrl')));
    expect(dialog, isNot(contains('NetworkPrivacySettingsDraft(')));
  });

  test('security dialogs are split by user flow', () {
    final settings = Directory('lib/src/features/settings');
    final barrel = File('${settings.path}/security_dialogs.dart');
    final source = barrel.readAsStringSync();

    for (final name in <String>[
      'advanced_security_dialog',
      'alternate_unlock_dialog',
      'master_password_dialog',
      'settings_error',
    ]) {
      expect(File('${settings.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("export '$name.dart';"));
    }

    expect(source, isNot(contains('AlertDialog')));
    expect(source, isNot(contains('TextField')));
    expect(source, isNot(contains('class AlternateUnlockDraft')));
  });

  test('alternate unlock dialog separates route, draft, form, and actions', () {
    final settings = Directory('lib/src/features/settings');
    final barrel = File('${settings.path}/alternate_unlock_dialog.dart');
    final route = File('${settings.path}/alternate_unlock_dialog_route.dart');
    final panel = File('${settings.path}/alternate_unlock_dialog_panel.dart');
    final form = File('${settings.path}/alternate_unlock_form.dart');
    final actions = File(
      '${settings.path}/alternate_unlock_dialog_actions.dart',
    );
    final draft = File('${settings.path}/alternate_unlock_draft.dart');
    final barrelSource = barrel.readAsStringSync();
    final routeSource = route.existsSync() ? route.readAsStringSync() : '';
    final panelSource = panel.existsSync() ? panel.readAsStringSync() : '';
    final formSource = form.existsSync() ? form.readAsStringSync() : '';
    final actionsSource = actions.existsSync()
        ? actions.readAsStringSync()
        : '';
    final draftSource = draft.existsSync() ? draft.readAsStringSync() : '';

    for (final file in <File>[route, panel, form, actions, draft]) {
      expect(file.existsSync(), isTrue);
    }

    expect(
      barrelSource,
      contains("export 'alternate_unlock_dialog_route.dart';"),
    );
    expect(barrelSource, contains("export 'alternate_unlock_draft.dart';"));
    expect(barrelSource, isNot(contains('StatefulWidget')));
    expect(barrelSource, isNot(contains('AlertDialog(')));
    expect(routeSource, contains('showAlternateUnlockDialog'));
    expect(routeSource, contains('AlternateUnlockDialogPanel('));
    expect(routeSource, isNot(contains('TextEditingController')));
    expect(panelSource, contains('class AlternateUnlockDialogPanel'));
    expect(panelSource, contains('AlternateUnlockForm('));
    expect(panelSource, contains('AlternateUnlockDialogActions('));
    expect(panelSource, isNot(contains('TextField(')));
    expect(panelSource, isNot(contains('FilledButton.icon')));
    expect(formSource, contains('class AlternateUnlockForm'));
    expect(formSource, contains('TextField('));
    expect(actionsSource, contains('class AlternateUnlockDialogActions'));
    expect(actionsSource, contains('FilledButton.icon'));
    expect(draftSource, contains('class AlternateUnlockDraft'));
  });

  test('keystore wallet dialogs are split by import and export flow', () {
    final wallets = Directory('lib/src/features/wallets/presentation');
    final barrel = File('${wallets.path}/keystore_wallet_dialogs.dart');
    final source = barrel.readAsStringSync();

    for (final name in <String>[
      'keystore_import_dialog',
      'keystore_export_dialog',
    ]) {
      expect(File('${wallets.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("export '$name.dart';"));
    }

    expect(source, isNot(contains('AlertDialog')));
    expect(source, isNot(contains('TextField')));
    expect(source, isNot(contains('JsonEncoder')));
  });

  test('multisig import delegates form layout and owner parsing', () {
    final multisig = Directory('lib/src/features/multisig/presentation');
    final dialog = File('${multisig.path}/multisig_import_dialog.dart');
    final source = dialog.readAsStringSync();
    final form = File('${multisig.path}/multisig_import_form.dart');
    final parser = File('${multisig.path}/multisig_owner_parser.dart');
    final formSource = form.existsSync() ? form.readAsStringSync() : '';

    expect(form.existsSync(), isTrue);
    expect(parser.existsSync(), isTrue);
    expect(source, contains("import 'multisig_import_form.dart';"));
    expect(formSource, contains("import 'multisig_owner_parser.dart';"));

    expect(source, contains('showImportMultisigDialog'));
    expect(source, contains('MultisigImportForm'));
    expect(source, isNot(contains('StatefulBuilder')));
    expect(source, isNot(contains('TextEditingController')));
    expect(source, isNot(contains('DropdownButtonFormField')));
    expect(source, isNot(contains('TextField')));
    expect(source, isNot(contains('_parseOwners')));
  });

  test('multisig import form delegates type and account fields', () {
    final multisig = Directory('lib/src/features/multisig/presentation');
    final form = File('${multisig.path}/multisig_import_form.dart');
    final source = form.readAsStringSync();

    for (final name in <String>[
      'multisig_import_account_fields',
      'multisig_import_type_fields',
    ]) {
      expect(File('${multisig.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("import '$name.dart';"));
    }

    expect(source, contains('MultisigImportTypeFields('));
    expect(source, contains('MultisigImportAccountFields('));
    expect(source, isNot(contains('DropdownButtonFormField')));
    expect(source, isNot(contains('TextField(')));
    expect(source, isNot(contains('InputDecoration(')));
  });

  test('multisig screen delegates body rows and app bar actions', () {
    final multisig = Directory('lib/src/features/multisig/presentation');
    final screen = File('${multisig.path}/multisig_screen.dart');
    final source = screen.readAsStringSync();

    for (final name in <String>[
      'multisig_screen_actions',
      'multisig_screen_body',
    ]) {
      expect(File('${multisig.path}/$name.dart').existsSync(), isTrue);
      expect(source, contains("import '$name.dart';"));
    }

    expect(source, contains('MultisigScreenActions('));
    expect(source, contains('MultisigScreenBody('));
    expect(source, isNot(contains('MultisigAccountTile(')));
    expect(source, isNot(contains('MultisigProposalTile(')));
    expect(source, isNot(contains('ListTile(')));
    expect(source, isNot(contains('IconButton(')));
    expect(source, isNot(contains('StaggeredListItem(')));
  });

  test('wallet workspace controller delegates state and action groups', () {
    final wallets = Directory('lib/src/features/wallets');
    final controller = File(
      '${wallets.path}/wallet_workspace_controller.dart',
    ).readAsStringSync();

    for (final name in <String>[
      'wallet_workspace_state',
      'wallet_workspace_wallet_selection',
      'wallet_workspace_wallet_mutations',
      'wallet_workspace_asset_actions',
    ]) {
      expect(File('${wallets.path}/$name.dart').existsSync(), isTrue);
      expect(controller, contains("part '$name.dart';"));
    }

    expect(controller, isNot(contains('class WalletWorkspaceState')));
    expect(controller, contains('WalletWorkspaceWalletSelection'));
    expect(controller, contains('WalletWorkspaceWalletMutations'));
    expect(controller, isNot(contains('_clients.wallets.create')));
    expect(controller, isNot(contains('_clients.assets.refresh')));
    expect(controller, isNot(contains('_clients.activity.sync')));
  });

  test(
    'wallet workspace wallet actions are split by selection and mutation flow',
    () {
      final wallets = Directory('lib/src/features/wallets');
      final selection = File(
        '${wallets.path}/wallet_workspace_wallet_selection.dart',
      );
      final mutations = File(
        '${wallets.path}/wallet_workspace_wallet_mutations.dart',
      );
      final oldActions = File(
        '${wallets.path}/wallet_workspace_wallet_actions.dart',
      );
      final selectionSource = selection.existsSync()
          ? selection.readAsStringSync()
          : '';
      final mutationSource = mutations.existsSync()
          ? mutations.readAsStringSync()
          : '';

      expect(oldActions.existsSync(), isFalse);
      expect(selection.existsSync(), isTrue);
      expect(mutations.existsSync(), isTrue);
      expect(selectionSource, contains('mixin WalletWorkspaceWalletSelection'));
      expect(selectionSource, contains('Future<void> loadWallets'));
      expect(selectionSource, contains('Future<void> loadWalletDetails'));
      expect(selectionSource, contains('Future<void> selectWallet'));
      expect(selectionSource, contains('void selectChain'));
      expect(selectionSource, isNot(contains('createWallet')));
      expect(selectionSource, isNot(contains('importPrivateKey')));
      expect(mutationSource, contains('mixin WalletWorkspaceWalletMutations'));
      expect(mutationSource, contains('Future<void> createWallet'));
      expect(mutationSource, contains('Future<void> importWallet'));
      expect(mutationSource, contains('Future<void> deleteWallet'));
      expect(mutationSource, isNot(contains('Future<void> loadWalletDetails')));
    },
  );

  test(
    'wallets screen delegates list rows, empty state, actions, and copy',
    () {
      final wallets = Directory('lib/src/features/wallets/presentation');
      final barrel = File('${wallets.path}/wallets_screen.dart');
      final route = File('${wallets.path}/wallets_screen_route.dart');
      final barrelSource = barrel.readAsStringSync();
      final routeSource = route.existsSync() ? route.readAsStringSync() : '';

      for (final name in <String>[
        'wallet_address_clipboard',
        'wallet_floating_actions',
        'wallet_list_items',
        'wallets_empty_state',
        'wallets_list',
        'wallets_screen_route',
      ]) {
        expect(File('${wallets.path}/$name.dart').existsSync(), isTrue);
        if ([
          'wallet_address_clipboard',
          'wallet_floating_actions',
          'wallets_list',
        ].contains(name)) {
          expect(routeSource, contains("import '$name.dart';"));
        }
      }

      expect(barrelSource, contains("export 'wallets_screen_route.dart';"));
      expect(barrelSource, isNot(contains('class WalletsScreen')));
      expect(routeSource, isNot(contains('ListTile(')));
      expect(routeSource, isNot(contains('FloatingActionButton')));
      expect(routeSource, isNot(contains('Clipboard.setData')));
      expect(routeSource, isNot(contains('ScaffoldMessenger.of')));
    },
  );

  test('transfer screen delegates payment loading and error presentation', () {
    final transfers = Directory('lib/src/features/transfers/presentation');
    final screen = File('${transfers.path}/transfer_screen.dart');
    final source = screen.readAsStringSync();

    expect(
      File('${transfers.path}/transfer_payment_loader.dart').existsSync(),
      isTrue,
    );
    expect(
      File('${transfers.path}/transfer_error_presenter.dart').existsSync(),
      isTrue,
    );
    expect(source, isNot(contains('PlatformException')));
    expect(source, isNot(contains('parsePaymentUri(payload)')));
    expect(source, isNot(contains('_firstAssetForChain')));
  });

  test('transfer screen delegates draft building and submission actions', () {
    final transfers = Directory('lib/src/features/transfers/presentation');
    final source = File(
      '${transfers.path}/transfer_screen.dart',
    ).readAsStringSync();

    expect(
      File('${transfers.path}/transfer_draft_builder.dart').existsSync(),
      isTrue,
    );
    expect(
      File('${transfers.path}/transfer_submission_actions.dart').existsSync(),
      isTrue,
    );
    expect(source, isNot(contains('TransferDraft(')));
    expect(source, isNot(contains('widget.assets.first')));
    expect(
      source,
      isNot(contains('ref.read(transferControllerProvider.notifier).send')),
    );
  });

  test('transfer screen route delegates body, QR actions, and feedback', () {
    final transfers = Directory('lib/src/features/transfers/presentation');
    final barrel = File('${transfers.path}/transfer_screen.dart');
    final route = File('${transfers.path}/transfer_screen_route.dart');
    final routeSource = route.existsSync() ? route.readAsStringSync() : '';
    final barrelSource = barrel.readAsStringSync();

    for (final name in <String>[
      'transfer_feedback',
      'transfer_qr_actions',
      'transfer_screen_body',
      'transfer_screen_route',
    ]) {
      expect(File('${transfers.path}/$name.dart').existsSync(), isTrue);
      if (name != 'transfer_screen_route') {
        expect(routeSource, contains("import '$name.dart';"));
      }
    }

    expect(barrelSource, contains("export 'transfer_screen_route.dart';"));
    expect(barrelSource, isNot(contains('class TransferScreen')));
    expect(routeSource, isNot(contains('Scaffold(')));
    expect(routeSource, isNot(contains('ScaffoldMessenger.of')));
    expect(routeSource, isNot(contains('scanQr()')));
    expect(routeSource, isNot(contains('pickQrImagePayload()')));
  });

  test('transfer screen route delegates stateful QR and submission flows', () {
    final transfers = Directory('lib/src/features/transfers/presentation');
    final route = File(
      '${transfers.path}/transfer_screen_route.dart',
    ).readAsStringSync();

    for (final name in <String>[
      'transfer_screen_qr_flow',
      'transfer_screen_submission_flow',
    ]) {
      expect(File('${transfers.path}/$name.dart').existsSync(), isTrue);
      expect(route, contains("part '$name.dart';"));
    }

    expect(route, contains('with TransferScreenQrFlow'));
    expect(route, contains('TransferScreenSubmissionFlow'));
    expect(route, isNot(contains('Future<void> _startCameraScan')));
    expect(route, isNot(contains('Future<void> _startImageImport')));
    expect(route, isNot(contains('Future<void> _previewTransfer')));
    expect(route, isNot(contains('Future<void> _sendTransfer')));
    expect(route, isNot(contains('TransferQrActions get _qrActions')));
    expect(route, isNot(contains('TransferSubmissionActions get')));
  });
}
