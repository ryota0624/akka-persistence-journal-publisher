## pre

```
rustup target add x86_64-unknown-linux-musl
```

# Run

```
docker run --rm -it -p 4566:4566 -p 4571:4571 -e "LAMBDA_FORWARD_URL=http://host.docker.internal:9001" -e "SERVICES=s3,lambda,dynamodb,sns,sqs"  localstack/localstack

aws s3 mb s3://lambda-code --endpoint-url http://localhost:4566
zip target/lambda/release/akka-persistence-journal-publisher.zip target/lambda/release/akka-persistence-journal-publisher 
aws s3 cp target/lambda/release/akka-persistence-journal-publisher.zip s3://lambda-code/akka-persistence-journal-publisher --endpoint-url http://localhost:4566

# dummy用のlambdaを作成
aws --profile localstack lambda create-function \
    --function-name=akka-persistence-journal-publisher --runtime=java11 --role=r1 --handler=handlers.JournalInsertHandler::handleRequest \
    --code S3Bucket=lambda-code,S3Key=akka-persistence-journal-publisher \
    --endpoint-url=http://localhost:4566 --region ap-northeast-1

aws lambda create-event-source-mapping \
    --function-name akka-persistence-journal-publisher \
    --event-source arn:aws:dynamodb:ap-northeast-1:000000000000:table/Journal/stream/2021-06-22T06:48:22.693 \
    --batch-size 10 \
    --starting-position TRIM_HORIZON \
    --endpoint http://localhost:4566 --region ap-northeast-1

make run_on_test_container

aws sns create-topic --name journal-publish-topic --endpoint http://localhost:4566 --region ap-northeast-1

aws sqs create-queue --queue-name journal-subscribe-queue --endpoint http://localhost:4566 --region ap-northeast-1

aws sns subscribe --topic-arn arn:aws:sns:ap-northeast-1:000000000000:journal-publish-topic \
    --protocol sqs --notification-endpoint http://localhost:4566/000000000000/journal-subscribe-queue --endpoint http://localhost:4566 --region ap-northeast-1

```
