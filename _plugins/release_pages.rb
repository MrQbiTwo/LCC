# Render version-specific reports from validated, explicit demonstration records.
module LccReports
  class Generator < Jekyll::Generator
    safe true
    priority :normal

    def generate(site)
      catalog = site.data.fetch('test_catalog')
      releases = site.data.fetch('releases')
      keys = catalog.flat_map { |group| group.fetch('tests').map { |test| [group['suite'], test['id']] } }
      raise 'Duplicate suite/case IDs' unless keys.uniq.length == keys.length
      raise 'Duplicate release slugs' unless releases.map { |r| r['slug'] }.uniq.length == releases.length

      releases.each do |release|
        raise 'Prototype records must be labeled demo' unless release['demo'] == true
        raise 'Invalid release slug' unless release['slug'].match?(/\A[a-z0-9-]+\z/)
        failures = release.fetch('failed_ids')
        raise 'Unknown failing case ID' unless (failures - keys.map(&:last)).empty?
        groups = catalog.map do |group|
          tests = group['tests'].map { |test| test.merge('result' => failures.include?(test['id']) ? 'Failed' : 'Passed') }
          group.merge('tests' => tests, 'total' => tests.length, 'passed' => tests.count { |t| t['result'] == 'Passed' })
        end
        total = groups.sum { |group| group['total'] }
        passed = groups.sum { |group| group['passed'] }
        release.merge!('path' => "/releases/#{release['slug']}", 'total' => total,
                       'passed' => passed, 'failed' => total - passed,
                       'percent' => (100.0 * passed / total).round(1))
        site.data.fetch('report_types').each do |report|
          page = Jekyll::PageWithoutAFile.new(site, site.source, "releases/#{release['slug']}/#{report['key']}", 'index.html')
          page.data.merge!('layout' => 'report', 'nav' => 'releases', 'title' => report['title'],
                           'release' => release, 'report_type' => report['key'], 'test_groups' => groups,
                           'features' => site.data.fetch('feature_examples'), 'platforms' => site.data.fetch('platforms'))
          site.pages << page
        end
      end
    end
  end
end
