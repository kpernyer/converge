swagger: "2.0"
info:
  title: Converge Runtime API
  description: API Gateway for Converge Runtime
  version: "1.0.0"
schemes:
  - https
produces:
  - application/json
x-google-backend:
  address: ${cloud_run_url}
  jwt_audience: ${cloud_run_url}
  protocol: h2
securityDefinitions:
  firebase:
    authorizationUrl: ""
    flow: implicit
    type: oauth2
    x-google-issuer: "https://securetoken.google.com/${project_id}"
    x-google-jwks_uri: "https://www.googleapis.com/service_accounts/v1/metadata/x509/securetoken@system.gserviceaccount.com"
    x-google-audiences: "${project_id}"
paths:
  /health:
    get:
      operationId: health
      summary: Health check
      responses:
        "200":
          description: Healthy
  /ready:
    get:
      operationId: ready
      summary: Readiness check
      responses:
        "200":
          description: Ready
  /api/v1/jobs:
    post:
      operationId: createJob
      summary: Submit a job
      security:
        - firebase: []
      responses:
        "200":
          description: Job completed
        "401":
          description: Unauthorized
  /api/v1/validate-rules:
    post:
      operationId: validateRules
      summary: Validate Converge Rules
      security:
        - firebase: []
      responses:
        "200":
          description: Validation result
        "401":
          description: Unauthorized
