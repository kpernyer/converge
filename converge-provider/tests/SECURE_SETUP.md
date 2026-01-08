# Secure API Key Setup for Integration Tests

## Quick Start

1. **Copy the example file**:
   ```bash
   cp ../../.env.example ../../.env
   ```

2. **Edit `.env`** and add your API key:
   ```bash
   ANTHROPIC_API_KEY=sk-ant-api03-your-actual-key-here
   ```

3. **Load the environment** (choose one method):

   **Option A: Manual load** (each terminal session):
   ```bash
   export $(cat ../../.env | xargs)
   ```

   **Option B: Use direnv** (automatic):
   ```bash
   # Install direnv first: brew install direnv
   # Then create .envrc in project root:
   echo "dotenv .env" > ../../.envrc
   direnv allow
   ```

4. **Run tests**:
   ```bash
   cargo test --test integration_prompt_formatting -- --ignored
   ```

## Verify Setup

```bash
# Check if key is set (should show your key)
echo $ANTHROPIC_API_KEY

# Run a single test to verify
cargo test --test integration_prompt_formatting test_format_correctness_edn_vs_plain -- --ignored
```

## Security Notes

- ✅ `.env` is already in `.gitignore` - safe to create
- ✅ Never commit `.env` to git
- ✅ Never share your API key
- ✅ Rotate keys if accidentally exposed

See `docs/development/SECURE_API_KEY_SETUP.md` for more options.

