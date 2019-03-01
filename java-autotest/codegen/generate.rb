# gem 'graphql_java_gen', '=0.1.1'
# gem 'graphql_schema', '=0.1.1'

require 'graphql_java_gen'
require 'graphql_schema'
require 'json'

introspection_result = File.read("codegen/schema.json")
schema = GraphQLSchema.new(JSON.parse(introspection_result))

GraphQLJavaGen.new(schema,
  package_name: "GraphQL",
  nest_under: 'GraphQLSchema',
  custom_scalars: [
    GraphQLJavaGen::Scalar.new(
      type_name: 'Decimal',
      java_type: 'BigDecimal',
      deserialize_expr: ->(expr) { "new BigDecimal(jsonAsString(#{expr}, key))" },
      imports: ['java.math.BigDecimal'],
    ),
  ]
).save("#{Dir.pwd}/src/main/java/GraphQL/GraphQLSchema.java")