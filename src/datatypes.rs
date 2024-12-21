use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::Date;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Outcome {
    Unspecified,
    Ok,
    Failed,
    DeadlineExceeded,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    Unspecified,
    Python,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    Unspecified,
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    Unspecified,
    HateSpeech,
    DangerousContent,
    Harassment,
    SexuallyExplicit,
    CivicIntegrity,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HarmBlockMethod {
    #[serde(rename = "HARM_BLOCK_METHOD_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "SEVERITY")]
    Severity,
    #[serde(rename = "PROBABILITY")]
    Probability,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    Unspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
    Off,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
    Unspecified,
    Dynamic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    Unspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
    Blocklist,
    ProhibitedContent,
    Spii,
    MalformedFunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "NEGLIGIBLE")]
    Negligible,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmSeverity {
    Unspecified,
    HarmSeverityNegligible,
    HarmSeverityLow,
    HarmSeverityMedium,
    HarmSeverityHigh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockedReason {
    Unspecified,
    Safety,
    Other,
    Blocklist,
    ProhibitedContent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DynamicRetrievalConfigMode {
    Unspecified,
    ModeDynamic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FunctionCallingConfigMode {
    #[serde(rename = "MODE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "NONE")]
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaResolution {
    Unspecified,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaskReferenceMode {
    MaskModeDefault,
    MaskModeUserProvided,
    MaskModeBackground,
    MaskModeForeground,
    MaskModeSemantic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ControlReferenceType {
    ControlTypeDefault,
    ControlTypeCanny,
    ControlTypeScribble,
    ControlTypeFaceMesh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubjectReferenceType {
    SubjectTypeDefault,
    SubjectTypePerson,
    SubjectTypeAnimal,
    SubjectTypeProduct,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoMetadata {
    pub end_offset: Option<String>,
    pub start_offset: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeExecutionResult {
    pub outcome: Outcome,
    pub output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutableCode {
    pub code: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub file_uri: String,
    pub mime_type: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub id: Option<String>,
    pub args: Option<serde_json::Value>,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionResponse {
    pub id: Option<String>,
    pub name: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blob {
    pub data: Vec<u8>,
    pub mime_type: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    pub video_metadata: Option<VideoMetadata>,
    pub code_execution_result: Option<CodeExecutionResult>,
    pub executable_code: Option<ExecutableCode>,
    pub file_data: Option<FileData>,
    pub function_call: Option<FunctionCall>,
    pub function_response: Option<FunctionResponse>,
    pub inline_data: Option<Blob>,
    pub text: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub parts: Option<Vec<Part>>,
    pub role: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schema {
    pub min_items: Option<String>,
    pub example: Option<serde_json::Value>,
    pub property_ordering: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub minimum: Option<f64>,
    pub default: Option<serde_json::Value>,
    pub any_of: Option<Vec<Schema>>,
    pub max_length: Option<String>,
    pub title: Option<String>,
    pub min_length: Option<String>,
    pub min_properties: Option<String>,
    pub max_items: Option<String>,
    pub maximum: Option<f64>,
    pub nullable: Option<bool>,
    pub max_properties: Option<String>,
    pub r#type: Option<Type>,
    pub description: Option<String>,
    pub r#enum: Option<Vec<String>>,
    pub format: Option<String>,
    pub items: Option<Box<Schema>>,
    pub properties: Option<std::collections::HashMap<String, Schema>>,
    pub required: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetySetting {
    pub method: Option<HarmBlockMethod>,
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDeclaration {
    pub response: Option<Schema>,
    pub description: Option<String>,
    pub name: String,
    pub parameters: Option<Schema>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSearch {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicRetrievalConfig {
    pub mode: Option<DynamicRetrievalConfigMode>,
    pub dynamic_threshold: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSearchRetrieval {
    pub dynamic_retrieval_config: Option<DynamicRetrievalConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexAISearch {
    pub datastore: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexRAGStoreRAGResource {
    pub rag_corpus: Option<String>,
    pub rag_file_ids: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexRAGStore {
    pub rag_corpora: Option<Vec<String>>,
    pub rag_resources: Option<Vec<VertexRAGStoreRAGResource>>,
    pub similarity_top_k: Option<i64>,
    pub vector_distance_threshold: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Retrieval {
    pub vertex_ai_search: Option<VertexAISearch>,
    pub vertex_rag_store: Option<VertexRAGStore>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCodeExecution {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
    pub retrieval: Option<Retrieval>,
    pub google_search: Option<GoogleSearch>,
    pub google_search_retrieval: Option<GoogleSearchRetrieval>,
    pub code_execution: Option<ToolCodeExecution>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCallingConfig {
    pub mode: Option<FunctionCallingConfigMode>,
    pub allowed_function_names: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolConfig {
    pub function_calling_config: Option<FunctionCallingConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrebuiltVoiceConfig {
    pub voice_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoiceConfig {
    pub prebuilt_voice_config: Option<PrebuiltVoiceConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechConfig {
    pub voice_config: Option<VoiceConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfigRoutingConfigAutoRoutingMode {
    pub model_routing_preference: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfigRoutingConfigManualRoutingMode {
    pub model_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfigRoutingConfig {
    pub auto_mode: Option<GenerationConfigRoutingConfigAutoRoutingMode>,
    pub manual_mode: Option<GenerationConfigRoutingConfigManualRoutingMode>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateContentConfig {
    pub system_instruction: Option<Content>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<f64>,
    pub candidate_count: Option<i64>,
    pub max_output_tokens: Option<i64>,
    pub stop_sequences: Option<Vec<String>>,
    pub response_logprobs: Option<bool>,
    pub logprobs: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub seed: Option<i64>,
    pub response_mime_type: Option<String>,
    pub response_schema: Option<Schema>,
    pub routing_config: Option<GenerationConfigRoutingConfig>,
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub tools: Option<Vec<Tool>>,
    pub tool_config: Option<ToolConfig>,
    pub cached_content: Option<String>,
    pub response_modalities: Option<Vec<String>>,
    pub media_resolution: Option<MediaResolution>,
    pub speech_config: Option<SpeechConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateContentParameters {
    pub model: Option<String>,
    pub contents: Option<Vec<Content>>,
    pub config: Option<GenerateContentConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Citation {
    pub end_index: Option<i64>,
    pub license: Option<String>,
    pub publication_date: Option<Date>,
    pub start_index: Option<i64>,
    pub title: Option<String>,
    pub uri: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CitationMetadata {
    pub citations: Option<Vec<Citation>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroundingChunkRetrievedContext {
    pub text: Option<String>,
    pub title: Option<String>,
    pub uri: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroundingChunkWeb {
    pub title: Option<String>,
    pub uri: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroundingChunk {
    pub retrieved_context: Option<GroundingChunkRetrievedContext>,
    pub web: Option<GroundingChunkWeb>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Segment {
    pub end_index: Option<i64>,
    pub part_index: Option<i64>,
    pub start_index: Option<i64>,
    pub text: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroundingSupport {
    pub confidence_scores: Option<Vec<f64>>,
    pub grounding_chunk_indices: Option<Vec<i64>>,
    pub segment: Option<Segment>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetrievalMetadata {
    pub google_search_dynamic_retrieval_score: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchEntryPoint {
    pub rendered_content: Option<String>,
    pub sdk_blob: Option<Vec<u8>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroundingMetadata {
    pub grounding_chunks: Option<Vec<GroundingChunk>>,
    pub grounding_supports: Option<Vec<GroundingSupport>>,
    pub retrieval_metadata: Option<RetrievalMetadata>,
    pub retrieval_queries: Option<Vec<String>>,
    pub search_entry_point: Option<SearchEntryPoint>,
    pub web_search_queries: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogprobsResultCandidate {
    pub log_probability: Option<f64>,
    pub token: Option<String>,
    pub token_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogprobsResultTopCandidates {
    pub candidates: Option<Vec<LogprobsResultCandidate>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogprobsResult {
    pub chosen_candidates: Option<Vec<LogprobsResultCandidate>>,
    pub top_candidates: Option<Vec<LogprobsResultTopCandidates>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetyRating {
    pub blocked: Option<bool>,
    pub category: Option<HarmCategory>,
    pub probability: Option<HarmProbability>,
    pub probability_score: Option<f64>,
    pub severity: Option<HarmSeverity>,
    pub severity_score: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub content: Option<Content>,
    pub citation_metadata: Option<CitationMetadata>,
    pub finish_message: Option<String>,
    pub token_count: Option<i64>,
    pub avg_logprobs: Option<f64>,
    pub finish_reason: Option<FinishReason>,
    pub grounding_metadata: Option<GroundingMetadata>,
    pub index: Option<i64>,
    pub logprobs_result: Option<LogprobsResult>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateContentResponsePromptFeedback {
    pub block_reason: Option<BlockedReason>,
    pub block_reason_message: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateContentResponseUsageMetadata {
    pub cached_content_token_count: Option<i64>,
    pub candidates_token_count: Option<i64>,
    pub prompt_token_count: Option<i64>,
    pub total_token_count: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateContentResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub model_version: Option<String>,
    pub prompt_feedback: Option<GenerateContentResponsePromptFeedback>,
    pub usage_metadata: Option<GenerateContentResponseUsageMetadata>,
}
