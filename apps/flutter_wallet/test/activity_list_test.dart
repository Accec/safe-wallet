import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/features/activity/presentation/activity_list.dart';
import 'package:flutter_wallet/src/models.dart';

void main() {
  testWidgets('activity list renders parsed transfer details', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: ActivityList(
            activity: [
              ActivitySummary(
                chain: 'ethereum',
                txHash:
                    '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
                kind: 'native_transfer',
                status: 'pending',
                summary: 'Sent 1.25 ETH',
              ),
            ],
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Sent'), findsOneWidget);
    expect(find.text('-1.25 ETH'), findsOneWidget);
    expect(find.text('Pending'), findsOneWidget);
    expect(find.text('0x123456...90abcdef'), findsOneWidget);
    expect(find.text('native_transfer'), findsNothing);
  });
}
