# secrets2env
A tiny tool that converts AWS Secrets Manager API responses into environment variables

**Usage**

```
SECRET_ARN="arn:aws:secretsmanager:us-east-1:000000000000:secret:secret-name"
source <(aws secretsmanager get-secret-value --secret-id $SECRET_ARN | secrets2env)
```
