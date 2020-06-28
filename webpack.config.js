const path = require("path");
const nodeExternals = require("webpack-node-externals");

module.exports = {
  mode: "production",
  devtool: "source-map",
  entry: "./app/index.tsx",
  target: 'electron-renderer',
  output: {
    filename: "index.js",
    path: path.resolve(__dirname, "dist"),
    libraryTarget: 'commonjs2',
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  externals: [nodeExternals()],
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
