const path = require('path');

module.exports = {
    entry: './src/main.js',
    devtool: 'inline-source-map',
    devServer: {
        contentBase: './dist'
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/
            },
            {
                test: /\.rs$/,
                use: {
                    loader: 'rust-wasm-loader',
                    options: {
                        path: './build'
                    }
                }
            }
        ]
    },
    resolve: {
        extensions: ['.ts', '.js']
    },
    externals: {
        'fs': true,
        'path': true
    },
    output: {
        filename: 'webarc.js',
        path: path.resolve(__dirname, 'dist')
    },
    mode: 'development'
};
