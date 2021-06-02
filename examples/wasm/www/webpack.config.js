const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  devtool: "source-map",
  plugins: [
    new CopyWebpackPlugin({patterns: ['index.html', '../README.md', "build-info.json"]})
  ],
  experiments: {
    syncWebAssembly: true,
  },
};
