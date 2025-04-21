import * as cdk from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as scheduler from 'aws-cdk-lib/aws-scheduler';
import * as targets from 'aws-cdk-lib/aws-scheduler-targets';
import { Construct } from 'constructs';

export class FootbotInfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const lambdaFn = new lambda.Function(this, 'FootbotService', {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      code: lambda.Code.fromAsset('../footbot_service/target/lambda/footbot_service/bootstrap.zip'),
      handler: 'bootstrap',
      memorySize: 128,
      timeout: cdk.Duration.seconds(5),
      architecture: lambda.Architecture.ARM_64,
    });

    const target = new targets.LambdaInvoke(lambdaFn);

    new scheduler.Schedule(this, 'FootbotSchedule', {
      schedule: scheduler.ScheduleExpression.expression('cron(0 20 ? * MON *)', cdk.TimeZone.EUROPE_LONDON),
      target,
      description: 'Trigger sending messages at 20:00 UK time',
    });
  }
}
