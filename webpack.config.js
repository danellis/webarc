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
                use: [
                    {
                        loader: 'wasm-loader',
                    },
                    {
                        loader: 'rust-native-wasm-loader',
                        options: {
                            release: true
                        }
                    }
                ]
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
