Import-Module PSFzf

# ブランチ選択用関数
function SelectGitBranch {
    param(
        [string]$query = ""
    )

    $selectedBranch = git branch | ForEach-Object { $_.Trim() } | fzf --query="$query" --select-1

    if ($selectedBranch) {
        git checkout $selectedBranch
    }
}

# Ctrl+gでFzfセッションを開始
Set-PSReadlineKeyHandler -Key 'Ctrl+g' -ScriptBlock {
    Select-Git-Branch
}
