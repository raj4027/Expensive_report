module.exports = {
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          {
            loader: "css-loader",
            options: {
              sourceMap: false, // Disable source maps for CSS
            },
          },
        ],
      },
    ],
  },
};
