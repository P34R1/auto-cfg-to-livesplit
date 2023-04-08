pub struct FileParsed<'cont> {
  pub category_name: String,
  pub splits: Vec<&'cont str>,
  pub file_name: &'cont str,
}

pub fn create_xml(file: &FileParsed) -> String {
  format!(
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<Run version=\"1.7.0\">
  <GameIcon />
  <GameName>Portal 2</GameName>
  <CategoryName>{} - CM</CategoryName>
  <LayoutPath>
  </LayoutPath>
  <Metadata>
    <Run id=\"\" />
    <Platform usesEmulator=\"False\">PC</Platform>
    <Region>
    </Region>
    <Variables />
  </Metadata>
  <Offset>00:00:00</Offset>
  <AttemptCount>0</AttemptCount>
  <AttemptHistory />
{}
  <AutoSplitterSettings>
    <Version>2.1.7</Version>
    <AutoSplitEnabled>True</AutoSplitEnabled>
    <SplitInterval>1</SplitInterval>
    <Start>True</Start>
    <Split>True</Split>
    <CustomSettings />
  </AutoSplitterSettings>
</Run>",
    file.category_name,
    create_segments(&file.splits)
  )
}

fn create_segments(splits: &Vec<&str>) -> String {
  let start_string = "<Segments>";
  let end_string = "</Segments>";
  let mut return_string = format!("  {}", start_string);
  for name in splits {
    return_string = format!(
      "{}
    <Segment>
      <Name>{}</Name>
      <Icon />
      <SplitTimes>
        <SplitTime name=\"Personal Best\" />
      </SplitTimes>
      <BestSegmentTime />
      <SegmentHistory />
    </Segment>",
      return_string, name
    );
  }
  return_string = format!(
    "{}
  {}",
    return_string, end_string
  );
  return_string
}
