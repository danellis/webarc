const path = require('path');

module.exports = {
    entry: './src/start.ts',
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
            }
        ]
    },
    resolve: {
        extensions: ['.ts', '.js']
    },
    output: {
        filename: 'webarc.js',
        path: path.resolve(__dirname, 'dist')
    },
    mode: 'development'
};
