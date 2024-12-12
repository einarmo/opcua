# TODO

This is a list of things that are known to be missing, or ideas that could be implemented. Feel free to pick up any of these if you wish to contribute.

 - Optimize the `Chunker`. It currently allocates too much, and it has some other inefficiencies.
 - Flesh out the server and client SDK with tooling for ease if use.
   - Make it even easier to implement custom node managers.
   - Write a generic `Browser` for the client, to make it easier to recursively browse node hierarchies. This should be made super flexible, perhaps a trait based approach where the browser is generic over something that handles the response from a browse request and returns what nodes to browse next...
   - Automate fetching data type definitions from the server for custom structs.
 - Support for StructureWithOptionalFields and Union in the encoding macros.
 - Implement Part 4 7.41.2.3, encrypted secrets. We currently only support legacy secrets. We should also support more encryption algorithms for secrets.
 - Write some form of support for IssuedToken based authentication on the client.
 - Implement a better framework for security checks on the server.
 - Write a sophisticated server example with a persistent store. This would be a great way to verify the flexibility of the server.
 - Write some "bad ideas" servers, it would be nice to showcase how flexible this is.
 - Re-implement XML. The current approach using roxmltree is easy to write, but not actually what we need if we really wanted to implement OPC-UA XML encoding. A stream-based low level XML parser like `quick-xml` would probably be a better option. An implementation could probably borrow a lot from the JSON implementation.
 - Write a framework for method calls. The foundation for this has been laid with `TryFromVariant`, if we really wanted to we could use clever trait magic to let users simply define a rust method that takes in values that each implement a trait `MethodArg`, with a blanket impl for `TryFromVariant`, and return a tuple of results. Could be really powerful, but methods are a little niche.
 - Implement `Query`. I never got around to this, because the service is just so complex. Currently there is no way to actually implement it, since it won't work unless _all_ node managers implement it, and the core node managers don't.
 - Look into running certain services concurrently. Currently they are sequential because that makes everything much simpler, but the services that don't have any cross node-manager interaction could run on all node managers concurrently.
