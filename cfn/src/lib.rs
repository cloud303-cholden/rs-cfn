use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Template {
    #[serde(rename = "AWSTemplateFormatVersion")]
    pub aws_template_format_version: Option<String>,
    pub description: Option<String>,
    pub parameters: Option<Parameters>,
    pub metadata: Option<Metadata>,
    pub rules: Option<Rules>,
    pub mappings: Option<Mappings>,
    pub conditions: Option<Conditions>,
    pub transform: Option<Transform>,
    pub resources: Option<Resources>,
    pub outputs: Option<Outputs>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    #[serde(rename = "Type")]
    pub ty: ParameterType,
    pub description: Option<String>,
    pub allowed_pattern: Option<String>,
    pub allowed_values: Option<Vec<Value>>,
    pub default: Option<Value>,
    pub no_echo: Option<bool>,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
}
pub type Parameters = HashMap<String, Parameter>;

#[derive(Debug, Deserialize, Serialize)]
pub enum ParameterType {
    String,
    Number,
    CommaDelimitedList,
    #[serde(rename = "List<Number>")]
    ListNumber,
    #[serde(rename = "AWS::EC2::AvailabilityZone::Name")]
    AwsEc2AvailabilityZoneName,
    #[serde(rename = "AWS::EC2::Image::Id")]
    AwsEc2ImageId,
    #[serde(rename = "AWS::EC2::Instance::Id")]
    AwsEc2InstanceId,
    #[serde(rename = "AWS::EC2::KeyPair::KeyName")]
    AwsEc2KeyPairKeyName,
    #[serde(rename = "AWS::EC2::SecurityGroup::GroupName")]
    AwsEc2SecurityGroupGroupName,
    #[serde(rename = "AWS::EC2::SecurityGroup::Id")]
    AwsEc2SecurityGroupId,
    #[serde(rename = "AWS::EC2::Subnet::Id")]
    AwsEc2SubnetId,
    #[serde(rename = "AWS::EC2::Volume::Id")]
    AwsEc2VolumeId,
    #[serde(rename = "AWS::EC2::VPC::Id")]
    AwsEc2VpcId,
    #[serde(rename = "AWS::Route53::HostedZone::Id")]
    AwsRoute53HostedZoneId,
    #[serde(rename = "List<AWS::EC2::AvailabilityZone::Name>")]
    ListAwsEc2AvailabilityZoneName,
    #[serde(rename = "List<AWS::EC2::Image::Id>")]
    ListAwsEc2ImageId,
    #[serde(rename = "List<AWS::EC2::Instance::Id>")]
    ListAwsEc2InstanceId,
    #[serde(rename = "List<AWS::EC2::SecurityGroup::GroupName>")]
    ListAwsEc2SecurityGroupGroupName,
    #[serde(rename = "List<AWS::EC2::SecurityGroup::Id>")]
    ListAwsEc2SecurityGroupId,
    #[serde(rename = "List<AWS::EC2::Subnet::Id>")]
    ListAwsEc2SubnetId,
    #[serde(rename = "List<AWS::EC2::Volume::Id>")]
    ListAwsEc2VolumeId,
    #[serde(rename = "List<AWS::EC2::VPC::Id>")]
    ListAwsEc2VpcId,
    #[serde(rename = "List<AWS::Route53::HostedZone::Id>")]
    ListAwsRoute53HostedZoneId,
    #[serde(rename = "AWS::SSM::Parameter::Name")]
    AwsSsmParameterName,
    #[serde(rename = "AWS::SSM::Parameter::Value<String>")]
    AwsSsmParameterValueString,
    #[serde(rename = "AWS::SSM::Parameter::Value<CommaDelimitedList>")]
    AwsSsmParameterValueCommaDelimitedList,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::AvailabilityZone::Name>")]
    AwsSsmParameterValueAwsEc2AvailabilityZoneName,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::Image::Id>")]
    AwsSsmParameterValueAwsEc2ImageId,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::Instance::Id>")]
    AwsSsmParameterValueAwsEc2InstanceId,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::KeyPair::KeyName>")]
    AwsSsmParameterValueAwsEc2KeyPairKeyName,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::SecurityGroup::GroupName>")]
    AwsSsmParameterValueAwsEc2SecurityGroupGroupName,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::SecurityGroup::Id>")]
    AwsSsmParameterValueAwsEc2SecurityGroupId,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::Subnet::Id>")]
    AwsSsmParameterValueAwsEc2SubnetId,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::Volume::Id>")]
    AwsSsmParameterValueAwsEc2VolumeId,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::EC2::VPC::Id>")]
    AwsSsmParameterValueAwsEc2VpcId,
    #[serde(rename = "AWS::SSM::Parameter::Value<AWS::Route53::HostedZone::Id>")]
    AwsSsmParameterValueAwsRoute53HostedZoneId,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::AvailabilityZone::Name>>")]
    AwsSsmParameterValueListAwsEc2AvailabilityZoneName,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::Image::Id>>")]
    AwsSsmParameterValueListAwsEc2ImageId,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::Instance::Id>>")]
    AwsSsmParameterValueListAwsEc2InstanceId,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::SecurityGroup::GroupName>>")]
    AwsSsmParameterValueListAwsEc2SecurityGroupGroupName,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::SecurityGroup::Id>>")]
    AwsSsmParameterValueListAwsEc2SecurityGroupId,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::Subnet::Id>>")]
    AwsSsmParameterValueListAwsEc2SubnetId,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::Volume::Id>>")]
    AwsSsmParameterValueListAwsEc2VolumeId,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::EC2::VPC::Id>>")]
    AwsSsmParameterValueListAwsEc2VpcId,
    #[serde(rename = "AWS::SSM::Parameter::Value<List<AWS::Route53::HostedZone::Id>>")]
    AwsSsmParameterValueListAwsRoute53HostedZoneId,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    #[serde(rename = "AWS::CloudFormation::Init")]
    pub aws_cloudformation_init: Option<AwsCloudFormationInit>,
    #[serde(rename = "AWS::CloudFormation::Interface")]
    pub aws_cloudformation_interface: Option<AwsCloudFormationInterface>,
    #[serde(rename = "AWS::CloudFormation::Designer")]
    pub aws_cloudformation_designer: Option<AwsCloudFormationDesigner>,
    #[serde(flatten)]
    pub custom: Option<HashMap<String, Value>>,
}
pub type AwsCloudFormationInit = Value;
pub type AwsCloudFormationDesigner = Value;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AwsCloudFormationInterface {
    pub parameter_groups: Option<ParameterGroups>,
    pub parameter_labels: Option<ParameterLabels>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterGroup {
    pub label: Option<ParameterLabel>,
    pub parameters: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterLabel {
    pub default: Option<String>,
}
pub type ParameterGroups = Vec<ParameterGroup>;
pub type ParameterLabels = HashMap<String, ParameterLabel>;

pub type Resource = Value;
pub type Resources = HashMap<String, Resource>;

pub type Rules = Value;
pub type Mappings = Value;
pub type Conditions = Value;
pub type Transform = Value;
pub type Outputs = Value;

pub enum RuleFunction {
    And,
    Contains,
    EachMemberEquals,
    EachMemberIn,
    Equals,
    If,
    Not,
    Or,
    RefAll,
    ValueOf,
    ValueOfAll,
}

impl From<&str> for RuleFunction {
    fn from(value: &str) -> Self {
        match value {
            "Fn::And" => Self::And,
            "Fn::Contains" => Self::Contains,
            "Fn::EachMemberEquals" => Self::EachMemberEquals,
            "Fn::EachMemberIn" => Self::EachMemberIn,
            "Fn::Equals" => Self::Equals,
            "Fn::If" => Self::If,
            "Fn::Not" => Self::Not,
            "Fn::Or" => Self::Or,
            "Fn::RefAll" => Self::RefAll,
            "Fn::ValueOf" => Self::ValueOf,
            "Fn::ValueOfAll" => Self::ValueOfAll,
            _ => unimplemented!(),
        }
    }
}
