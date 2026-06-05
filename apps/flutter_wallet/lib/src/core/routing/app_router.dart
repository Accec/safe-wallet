import 'package:flutter/widgets.dart';
import 'package:go_router/go_router.dart';

GoRouter createSafeWalletRouter({required Widget child}) {
  return GoRouter(
    initialLocation: '/',
    routes: [GoRoute(path: '/', builder: (context, state) => child)],
  );
}
