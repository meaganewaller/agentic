# Vendor Adapters

Adapters transform merged config into vendor format.

They are registered via the adapter registry.

Adding a new vendor:

1. Implement `VendorAdapter`
2. Add to registry
3. Add schema support
4. Add snapshot test
