const path = require("path");
const nodeExternals = require("webpack-node-externals");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "development",
  devtool: "cheap-module-eval-source-map",
  entry: "./app/index.tsx",
  target: "electron-renderer",
  output: {
    filename: "index.js",
    path: path.resolve(__dirname, "dist"),
    libraryTarget: "commonjs2",
  },
  resolve: {
    modules: ['node_modules'],
    extensions: [".tsx", ".ts", ".js", ".wasm"],
  },
  externals: [nodeExternals()],
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.'),
    }),
  ],
  module: {
    rules: [
      {
        test: /\.(js|ts|tsx)$/,
        use: [
          {
            loader: "babel-loader",
            options: {
              babelrc: false,
              configFile: false,
              presets: [
                [
                  "@babel/preset-env",
                  {
                    useBuiltIns: "entry",
                    corejs: 3,
                    modules: false,
                    exclude: ["transform-typeof-symbol"],
                  },
                ],
                [
                  "@babel/preset-react",
                  {
                    development: false,
                    useBuiltIns: true,
                  },
                ],
                "@babel/preset-typescript",
              ],
              plugins: [
                [
                  "@babel/plugin-transform-runtime",
                  {
                    corejs: false,
                    version: require("@babel/runtime/package.json").version,
                    regenerator: true,
                    usESModules: true,
                  },
                ],
                "@babel/plugin-syntax-dynamic-import",
              ],
              cacheDirectory: true,
              cacheCompression: false,
            },
          },
        ],
      },
    ],
  },
};
