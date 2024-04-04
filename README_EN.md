# LeetCode Daily Challenge Worker

This is a Cloudflare worker that runs daily at 0:10 UTC-0. Its function is to fetch the daily challenge from LeetCode using the LeetCode GraphQL API. Then, it processes the data and sends it in Markdown format to an endpoint defined in the `ENDPOINT` environment variable. In our case, this endpoint corresponds to a Discord bot that creates a discussion thread where users can debate solutions and approaches to solve the daily challenge.

## How It Works

1. The worker is automatically activated at 0:10 UTC-0 thanks to a scheduled trigger in Cloudflare.
2. A call is made to the LeetCode GraphQL API to get the details of the daily challenge.
3. The challenge data is processed and converted to Markdown format.
4. A POST request is sent to the API defined in the `ENDPOINT` environment variable with the challenge data in Markdown format.
5. In our case, the `ENDPOINT` corresponds to a Discord bot that creates a discussion thread with the details of the challenge.

## Configuration

### Requirements

To build and deploy this project, you will need the following:

- [Rust](https://rust-lang.org)
- [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [worker-build](https://crates.io/crates/worker-build)
    - [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Environment Variables

- `ENDPOINT`: URL of the endpoint where the daily challenge data will be sent.

### Local Testing

To test the worker locally, you only need to configure the `ENDPOINT` environment variable in the `wrangler.toml` file. This is only necessary if you want to use a different endpoint than the one already configured.

### Automatic Deployment

This project is configured to deploy automatically using GitHub Actions workflows. For it to work correctly, you must configure the following secrets in GitHub:

- `CLOUDFLARE_ACCOUNT_ID`: Your Cloudflare account ID.
- `CLOUDFLARE_API_TOKEN`: Your Cloudflare API token.
- `ENDPOINT`: URL of the API to which the daily challenge data will be sent.
