import 'dart:convert';

import '../../wallet_api.dart';

class WalletProtocol {
  int _requestSequence = 0;

  Map<String, Object?> wrapRequest({
    required String domain,
    required String action,
    Map<String, Object?> payload = const {},
  }) {
    _requestSequence += 1;
    return <String, Object?>{
      'protocol_version': 2,
      'request_id': '$domain.$action.$_requestSequence',
      'domain': domain,
      'action': action,
      'payload': payload,
    };
  }

  Map<String, Object?> wrapLegacyCommand(Map<String, Object?> command) {
    final commandName = command['command'];
    if (commandName is! String) {
      throw const WalletApiException('Invalid native command');
    }
    final route = routeForLegacyCommand(commandName);
    final payload = Map<String, Object?>.from(command)..remove('command');
    return wrapRequest(
      domain: route.domain,
      action: route.action,
      payload: payload,
    );
  }

  Object? unwrapNativeResponse(Map<String, dynamic> response) {
    if (response['ok'] != true) {
      throw WalletApiException(
        response['error'] as String? ?? 'Native command failed',
      );
    }
    final body = jsonDecode(response['body_json'] as String);
    if (body is Map<String, dynamic> &&
        body.containsKey('request_id') &&
        body.containsKey('ok')) {
      if (body['ok'] != true) {
        final error = body['error'];
        if (error is Map<String, dynamic>) {
          throw WalletApiException(
            error['message'] as String? ?? 'Native command failed',
          );
        }
        throw const WalletApiException('Native command failed');
      }
      return body['data'];
    }
    return body;
  }
}

({String domain, String action}) routeForLegacyCommand(String command) {
  return switch (command) {
    'app_status' => (domain: 'app', action: 'status'),
    'set_master_password' => (domain: 'auth', action: 'set_master_password'),
    'unlock_app' => (domain: 'auth', action: 'unlock'),
    'set_duress_password' => (domain: 'auth', action: 'set_duress_password'),
    'update_biometric_unlock' => (
      domain: 'auth',
      action: 'update_biometric_unlock',
    ),
    'generate_mnemonic' => (domain: 'wallets', action: 'generate_mnemonic'),
    'create_wallet' => (domain: 'wallets', action: 'create'),
    'import_wallet' => (domain: 'wallets', action: 'import_mnemonic'),
    'import_private_key' => (domain: 'wallets', action: 'import_private_key'),
    'import_keystore' => (domain: 'wallets', action: 'import_keystore'),
    'export_keystore' => (domain: 'wallets', action: 'export_keystore'),
    'delete_wallet' => (domain: 'wallets', action: 'delete'),
    'list_wallets' => (domain: 'wallets', action: 'list'),
    'list_accounts' => (domain: 'wallets', action: 'list_accounts'),
    'list_assets' => (domain: 'assets', action: 'list'),
    'refresh_assets' => (domain: 'assets', action: 'refresh'),
    'discover_assets' => (domain: 'assets', action: 'discover'),
    'add_custom_token' => (domain: 'assets', action: 'add_custom_token'),
    'remove_custom_token' => (domain: 'assets', action: 'remove_custom_token'),
    'parse_payment_uri' => (domain: 'transfers', action: 'parse_payment_uri'),
    'preview_transfer' => (domain: 'transfers', action: 'preview'),
    'send_transfer' => (domain: 'transfers', action: 'send'),
    'list_activity' => (domain: 'activity', action: 'list'),
    'sync_activity' => (domain: 'activity', action: 'sync'),
    'import_multisig_account' => (domain: 'multisig', action: 'import_account'),
    'list_multisig_accounts' => (domain: 'multisig', action: 'list_accounts'),
    'create_multisig_proposal' => (
      domain: 'multisig',
      action: 'create_proposal',
    ),
    'list_multisig_proposals' => (domain: 'multisig', action: 'list_proposals'),
    'add_multisig_signature' => (domain: 'multisig', action: 'add_signature'),
    'get_network_privacy_settings' => (
      domain: 'network',
      action: 'get_privacy',
    ),
    'save_network_privacy_settings' => (
      domain: 'network',
      action: 'save_privacy',
    ),
    'test_proxy_connection' => (domain: 'network', action: 'test_proxy'),
    'list_network_settings' => (domain: 'network', action: 'list_settings'),
    'save_network_settings' => (domain: 'network', action: 'save_settings'),
    'update_chain_rpc' => (domain: 'network', action: 'update_chain_rpc'),
    'update_indexer_settings' => (domain: 'network', action: 'update_indexer'),
    _ => throw const WalletApiException('Invalid native command'),
  };
}
