import '../../../models.dart';

List<MultisigOwnerDraft> parseMultisigOwners(String value) {
  final owners = <MultisigOwnerDraft>[];
  for (final line in value.split('\n')) {
    final parts = line.split(',');
    if (parts.length != 2) {
      continue;
    }
    final address = parts[0].trim();
    final weight = int.tryParse(parts[1].trim());
    if (address.isEmpty || weight == null || weight <= 0) {
      continue;
    }
    owners.add(MultisigOwnerDraft(address: address, weight: weight));
  }
  return owners;
}
